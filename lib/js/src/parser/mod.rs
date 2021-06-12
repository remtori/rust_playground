mod error;
pub mod lexer;
pub mod token;

use core::panic;
use std::default::default;

use error::ParseError;
use lexer::Lexer;
use token::{Token, TokenKind};
use utils::prelude::*;

use crate::ast::*;

#[derive(Debug)]
pub struct Parser<'s> {
    lexer: Lexer<'s>,
    current_token: Token<'s>,
}

type Result<'s, T> = std::result::Result<T, ParseError<'s>>;

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            lexer: Lexer::new(source),
            current_token: Token::default(),
        }
    }

    pub fn parse_program(&mut self) -> Result<'s, Program> {
        self.consume();
        let mut program = Program::new();

        while !self.done() {
            if self.match_declaration() {
                program.add_statement(self.parse_declaration()?);
                self.consume_or_insert_semicolon();
            } else if self.match_statement() {
                program.add_statement(self.parse_statement()?);
                self.consume_or_insert_semicolon();
            } else {
                return Err(ParseError::unexpected(self.consume()));
            }
        }

        Ok(program)
    }

    fn parse_declaration(&mut self) -> Result<'s, Statement> {
        if self.match_variable_declaration() {
            Ok(Statement::VariableDeclaration(
                self.parse_variable_declaration()?,
            ))
        } else if self.current_token.kind() == TokenKind::Function {
            Ok(Statement::FunctionDeclaration(
                self.parse_function_declaration()?,
            ))
        } else {
            todo!("Unhandle: {:?}", self.current_token)
        }
    }

    fn parse_function_declaration(&mut self) -> Result<'s, FunctionDeclaration> {
        self.consume_token(TokenKind::Function)?;

        let ident = self.consume_token(TokenKind::Identifier)?;

        self.consume_token(TokenKind::ParenOpen)?;

        let mut params = Vec::new();
        loop {
            if self.current_token.kind() == TokenKind::ParenClose {
                break;
            }

            params.push(Identifier::new(
                self.consume_token(TokenKind::Identifier)?.value(),
            ));

            if self.current_token.kind() == TokenKind::ParenClose {
                break;
            }

            self.consume_token(TokenKind::Comma)?;
        }

        self.consume_token(TokenKind::ParenClose)?;

        let body = self.parse_block_statement()?;

        Ok(FunctionDeclaration::new(
            Identifier::new(ident.value()),
            params,
            body,
        ))
    }

    fn parse_block_statement(&mut self) -> Result<'s, BlockStatement> {
        let mut block_statement = BlockStatement::new();

        self.consume_token(TokenKind::CurlyOpen)?;
        loop {
            if self.current_token.kind() == TokenKind::CurlyClose {
                break;
            }

            block_statement.add_statement(self.parse_statement()?);
            self.consume_or_insert_semicolon();
        }
        self.consume_token(TokenKind::CurlyClose)?;

        Ok(block_statement)
    }

    fn parse_statement(&mut self) -> Result<'s, Statement> {
        Ok(if self.match_variable_declaration() {
            Statement::VariableDeclaration(self.parse_variable_declaration()?)
        } else if self.match_expression() {
            Statement::ExpressionStatement(self.parse_expression(0, Associativity::Right)?)
        } else if self.current_token.kind() == TokenKind::Return {
            self.consume();
            Statement::ReturnStatement(self.parse_expression(default(), default())?)
        } else {
            return Err(ParseError::unexpected(self.current_token));
        })
    }

    fn parse_variable_declaration(&mut self) -> Result<'s, VariableDeclaration> {
        let kind = match self.current_token.kind() {
            TokenKind::Const => DeclarationKind::Const,
            TokenKind::Let => DeclarationKind::Let,
            TokenKind::Var => DeclarationKind::Var,
            _ => unreachable!(),
        };
        self.consume();

        let mut vars = VariableDeclaration::new(kind);

        loop {
            let identifier = self.consume_token(TokenKind::Identifier)?;
            self.consume_token(TokenKind::Equals)?;
            let initializer = self.parse_expression(0, default())?;
            vars.add(Identifier::new(identifier.value()), initializer);

            if let TokenKind::Comma = self.current_token.kind() {
                self.consume();
            } else {
                break;
            }
        }

        Ok(vars)
    }

    fn parse_expression(
        &mut self,
        min_precedence: u32,
        associativity: Associativity,
    ) -> Result<'s, Expression> {
        let mut expr = self.parse_primary_expression()?;
        while self.match_secondary_expression() {
            let new_precedence = token_precedence(&self.current_token.kind());
            if (new_precedence < min_precedence)
                || (new_precedence == min_precedence && associativity == Associativity::Left)
            {
                break;
            }

            expr = self.parse_secondary_expression(
                expr,
                new_precedence,
                operator_associativity(&self.current_token.kind()),
            )?;
        }

        Ok(expr)
    }

    fn parse_primary_expression(&mut self) -> Result<'s, Expression> {
        Ok(match self.current_token.kind() {
            TokenKind::ParenOpen => {
                self.consume_token(TokenKind::ParenOpen)?;
                let expr = self.parse_expression(0, default())?;
                self.consume_token(TokenKind::ParenClose)?;
                expr
            }
            TokenKind::BoolLiteral => {
                Expression::Literal(Literal::Boolean(self.consume().bool_value()))
            }
            TokenKind::NullLiteral => Expression::Literal(Literal::Null),
            TokenKind::StringLiteral => {
                Expression::Literal(Literal::String(self.consume().string_value()))
            }
            TokenKind::NumericLiteral => {
                Expression::Literal(Literal::number_from_str(self.consume().value()))
            }
            TokenKind::Identifier => {
                Expression::Identifier(Identifier::new(self.consume().value()))
            }
            TokenKind::CurlyOpen => {
                self.consume_token(TokenKind::CurlyOpen)?;
                let expr = self.parse_object_expression()?;
                self.consume_token(TokenKind::CurlyClose)?;
                Expression::ObjectExpression(expr)
            }
            _ => todo!("Unhandle: {:?}", self.current_token),
        })
    }

    fn parse_secondary_expression(
        &mut self,
        lhs: Expression,
        min_precedence: u32,
        associativity: Associativity,
    ) -> Result<'s, Expression> {
        Ok(match self.current_token.kind() {
            TokenKind::Plus => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Addition,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Minus => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Subtraction,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Asterisk => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Multiplication,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Slash => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Division,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Percent => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Modulo,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::DoubleAsterisk => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::numeric(
                    NumericOp::Exponent,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Pipe => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::Or,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Ampersand => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::And,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::Caret => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::Xor,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::ShiftLeft => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::ShiftLeft,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::ShiftRight => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::ShiftRight,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::UnsignedShiftRight => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::bitwise(
                    BitwiseOp::UnsignedShiftRight,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::DoublePipe => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::new(
                    BinaryOp::BoolOr,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::DoubleAmpersand => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::new(
                    BinaryOp::BoolAnd,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::EqualsEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::Equal,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::ExclamationMarkEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::NotEqual,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::EqualsEqualsEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::StrictEqual,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::ExclamationMarkEqualsEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::StrictNotEqual,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::GreaterThan => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::GreaterThan,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::GreaterThanEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::GreaterThanOrEqual,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::LessThan => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::LessThan,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::LessThanEquals => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::LessThanOrEqual,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::In => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::In,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::InstanceOf => {
                self.consume();
                Expression::BinaryOperation(BinaryOperation::compare(
                    CompareOp::InstanceOf,
                    lhs,
                    self.parse_expression(min_precedence, associativity)?,
                ))
            }
            TokenKind::PlusEquals => self.parse_assignment_expression(
                AssignmentOp::AdditionAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::MinusEquals => self.parse_assignment_expression(
                AssignmentOp::SubtractionAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::AsteriskEquals => self.parse_assignment_expression(
                AssignmentOp::MultiplicationAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::SlashEquals => self.parse_assignment_expression(
                AssignmentOp::DivisionAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::PercentEquals => self.parse_assignment_expression(
                AssignmentOp::ModuloAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::DoubleAsteriskEquals => self.parse_assignment_expression(
                AssignmentOp::ExponentAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::AmpersandEquals => self.parse_assignment_expression(
                AssignmentOp::BitAndAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::PipeEquals => self.parse_assignment_expression(
                AssignmentOp::BitOrAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::CaretEquals => self.parse_assignment_expression(
                AssignmentOp::BitXorAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::ShiftLeftEquals => self.parse_assignment_expression(
                AssignmentOp::ShiftLeftAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::ShiftRightEquals => self.parse_assignment_expression(
                AssignmentOp::ShiftRightAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::UnsignedShiftRightEquals => self.parse_assignment_expression(
                AssignmentOp::UnsignedShiftRightAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::DoubleAmpersandEquals => self.parse_assignment_expression(
                AssignmentOp::BoolAndAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::DoublePipeEquals => self.parse_assignment_expression(
                AssignmentOp::BoolOrAssignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::Equals => self.parse_assignment_expression(
                AssignmentOp::Assignment,
                lhs,
                min_precedence,
                associativity,
            )?,
            TokenKind::ParenOpen => {
                self.parse_call_expression(lhs, min_precedence, associativity)?
            }
            _ => unreachable!(),
        })
    }

    fn parse_assignment_expression(
        &mut self,
        op: AssignmentOp,
        lhs: Expression,
        min_precedence: u32,
        associativity: Associativity,
    ) -> Result<'s, Expression> {
        assert!(matches!(
            self.current_token.kind(),
            TokenKind::Equals
                | TokenKind::PlusEquals
                | TokenKind::MinusEquals
                | TokenKind::AsteriskEquals
                | TokenKind::SlashEquals
                | TokenKind::PercentEquals
                | TokenKind::DoubleAsteriskEquals
                | TokenKind::AmpersandEquals
                | TokenKind::PipeEquals
                | TokenKind::CaretEquals
                | TokenKind::ShiftLeftEquals
                | TokenKind::ShiftRightEquals
                | TokenKind::UnsignedShiftRightEquals
                | TokenKind::DoubleAmpersandEquals
                | TokenKind::DoublePipeEquals
                | TokenKind::DoubleQuestionMarkEquals
        ));
        self.consume();

        if matches!(lhs, Expression::Identifier(_),) {
            Ok(Expression::BinaryOperation(BinaryOperation::assignment(
                op,
                lhs,
                self.parse_expression(min_precedence, associativity)?,
            )))
        } else {
            Err(ParseError::unexpected(self.current_token))
        }
    }

    fn parse_object_expression(&mut self) -> Result<'s, ObjectExpression> {
        let mut properties = Vec::new();
        loop {
            if self.current_token.kind() == TokenKind::CurlyClose {
                return Ok(ObjectExpression::new(properties));
            }

            let key = self.consume_token(TokenKind::Identifier)?;
            match self.current_token.kind() {
                TokenKind::Colon => {
                    self.consume();
                    let value = self.parse_expression(0, default())?;
                    properties.push(ObjectProperty::new(
                        Expression::Identifier(Identifier::new(key.value())),
                        Some(value),
                        ObjectPropertyKind::KeyValue,
                        false,
                    ));
                }
                TokenKind::Comma => {
                    properties.push(ObjectProperty::new(
                        Expression::Identifier(Identifier::new(key.value())),
                        None,
                        ObjectPropertyKind::KeyValue,
                        false,
                    ));
                }
                _ => return Err(ParseError::unexpected(self.current_token)),
            }

            if self.current_token.kind() == TokenKind::Comma {
                self.consume();
            }
        }
    }

    fn parse_call_expression(
        &mut self,
        lhs: Expression,
        min_precedence: u32,
        associativity: Associativity,
    ) -> Result<'s, Expression> {
        self.consume_token(TokenKind::ParenOpen)?;

        let mut args = Vec::new();
        loop {
            if self.current_token.kind() == TokenKind::ParenClose {
                break;
            }

            args.push(self.parse_expression(0, default())?);

            if self.current_token.kind() == TokenKind::Comma {
                self.consume();
            } else {
                break;
            }
        }

        self.consume_token(TokenKind::ParenClose)?;

        Ok(Expression::CallExpression(CallExpression::new(lhs, args)))
    }

    fn consume_or_insert_semicolon(&mut self) {
        if let TokenKind::Semicolon = self.current_token.kind() {
            self.consume();
        }
    }

    fn done(&self) -> bool {
        self.current_token.kind() == TokenKind::Eof
    }

    fn match_token(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind() == token_kind
    }

    fn match_declaration(&self) -> bool {
        matches!(
            self.current_token.kind(),
            TokenKind::Const | TokenKind::Class | TokenKind::Let | TokenKind::Function
        )
    }

    fn match_variable_declaration(&self) -> bool {
        matches!(
            self.current_token.kind(),
            TokenKind::Const | TokenKind::Let | TokenKind::Var
        )
    }

    fn match_statement(&self) -> bool {
        self.match_expression()
            || matches!(
                self.current_token.kind(),
                TokenKind::Break
                    | TokenKind::Continue
                    | TokenKind::CurlyOpen
                    | TokenKind::Debugger
                    | TokenKind::Do
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::Return
                    | TokenKind::Semicolon
                    | TokenKind::Switch
                    | TokenKind::Throw
                    | TokenKind::Try
                    | TokenKind::Var
                    | TokenKind::While
                    | TokenKind::With
            )
    }

    fn match_expression(&self) -> bool {
        self.match_unary_prefixed_expression()
            || matches!(
                self.current_token.kind(),
                TokenKind::BigIntLiteral
                    | TokenKind::BoolLiteral
                    | TokenKind::BracketOpen
                    | TokenKind::CurlyOpen
                    | TokenKind::Function
                    | TokenKind::Identifier
                    | TokenKind::New
                    | TokenKind::NullLiteral
                    | TokenKind::NumericLiteral
                    | TokenKind::ParenOpen
                    | TokenKind::RegexLiteral
                    | TokenKind::StringLiteral
                    | TokenKind::Super
                    | TokenKind::TemplateLiteralStart
                    | TokenKind::This
            )
    }

    fn match_unary_prefixed_expression(&self) -> bool {
        matches!(
            self.current_token.kind(),
            TokenKind::PlusPlus
                | TokenKind::MinusMinus
                | TokenKind::ExclamationMark
                | TokenKind::Tilde
                | TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Typeof
                | TokenKind::Void
                | TokenKind::Delete
        )
    }

    fn match_secondary_expression(&self) -> bool {
        matches!(
            self.current_token.kind(),
            TokenKind::Plus
                | TokenKind::PlusEquals
                | TokenKind::Minus
                | TokenKind::MinusEquals
                | TokenKind::Asterisk
                | TokenKind::AsteriskEquals
                | TokenKind::Slash
                | TokenKind::SlashEquals
                | TokenKind::Percent
                | TokenKind::PercentEquals
                | TokenKind::DoubleAsterisk
                | TokenKind::DoubleAsteriskEquals
                | TokenKind::Equals
                | TokenKind::EqualsEqualsEquals
                | TokenKind::ExclamationMarkEqualsEquals
                | TokenKind::EqualsEquals
                | TokenKind::ExclamationMarkEquals
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEquals
                | TokenKind::LessThan
                | TokenKind::LessThanEquals
                | TokenKind::ParenOpen
                | TokenKind::Period
                | TokenKind::BracketOpen
                | TokenKind::PlusPlus
                | TokenKind::MinusMinus
                | TokenKind::In
                | TokenKind::InstanceOf
                | TokenKind::QuestionMark
                | TokenKind::Ampersand
                | TokenKind::AmpersandEquals
                | TokenKind::Pipe
                | TokenKind::PipeEquals
                | TokenKind::Caret
                | TokenKind::CaretEquals
                | TokenKind::ShiftLeft
                | TokenKind::ShiftLeftEquals
                | TokenKind::ShiftRight
                | TokenKind::ShiftRightEquals
                | TokenKind::UnsignedShiftRight
                | TokenKind::UnsignedShiftRightEquals
                | TokenKind::DoubleAmpersand
                | TokenKind::DoubleAmpersandEquals
                | TokenKind::DoublePipe
                | TokenKind::DoublePipeEquals
                | TokenKind::DoubleQuestionMark
                | TokenKind::DoubleQuestionMarkEquals
        )
    }

    fn consume_token(&mut self, token_kind: TokenKind) -> Result<'s, Token<'s>> {
        if self.match_token(token_kind) {
            Ok(self.consume())
        } else {
            Err(ParseError::expect(token_kind, self.current_token))
        }
    }

    fn consume(&mut self) -> Token<'s> {
        let old_token = self.current_token;
        self.current_token = self.lexer.next_token();
        old_token
    }
}

#[derive(Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

impl Default for Associativity {
    fn default() -> Associativity {
        Associativity::Right
    }
}

fn operator_associativity(tk: &TokenKind) -> Associativity {
    match tk {
        TokenKind::Period
        | TokenKind::BracketOpen
        | TokenKind::ParenOpen
        | TokenKind::QuestionMarkPeriod
        | TokenKind::Asterisk
        | TokenKind::Slash
        | TokenKind::Percent
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::ShiftLeft
        | TokenKind::ShiftRight
        | TokenKind::UnsignedShiftRight
        | TokenKind::LessThan
        | TokenKind::LessThanEquals
        | TokenKind::GreaterThan
        | TokenKind::GreaterThanEquals
        | TokenKind::In
        | TokenKind::InstanceOf
        | TokenKind::EqualsEquals
        | TokenKind::ExclamationMarkEquals
        | TokenKind::EqualsEqualsEquals
        | TokenKind::ExclamationMarkEqualsEquals
        | TokenKind::Typeof
        | TokenKind::Void
        | TokenKind::Delete
        | TokenKind::Ampersand
        | TokenKind::Caret
        | TokenKind::Pipe
        | TokenKind::DoubleQuestionMark
        | TokenKind::DoubleAmpersand
        | TokenKind::DoublePipe
        | TokenKind::Comma => Associativity::Left,

        _ => Associativity::Right,
    }
}

fn token_precedence(tk: &TokenKind) -> u32 {
    match tk {
        TokenKind::Period
        | TokenKind::BracketOpen
        | TokenKind::ParenOpen
        | TokenKind::QuestionMarkPeriod => 20,

        TokenKind::New => 19,

        TokenKind::PlusPlus | TokenKind::MinusMinus => 18,

        TokenKind::ExclamationMark
        | TokenKind::Tilde
        | TokenKind::Typeof
        | TokenKind::Void
        | TokenKind::Delete
        | TokenKind::Await => 17,

        TokenKind::DoubleAsterisk => 16,

        TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => 15,

        TokenKind::Plus | TokenKind::Minus => 14,

        TokenKind::ShiftLeft | TokenKind::ShiftRight | TokenKind::UnsignedShiftRight => 13,

        TokenKind::LessThan
        | TokenKind::LessThanEquals
        | TokenKind::GreaterThan
        | TokenKind::GreaterThanEquals
        | TokenKind::In
        | TokenKind::InstanceOf => 12,

        TokenKind::EqualsEquals
        | TokenKind::ExclamationMarkEquals
        | TokenKind::EqualsEqualsEquals
        | TokenKind::ExclamationMarkEqualsEquals => 11,

        TokenKind::Ampersand => 10,

        TokenKind::Caret => 9,

        TokenKind::Pipe => 8,

        TokenKind::DoubleQuestionMark => 7,

        TokenKind::DoubleAmpersand => 6,

        TokenKind::DoublePipe => 5,

        TokenKind::QuestionMark => 4,

        TokenKind::Equals
        | TokenKind::PlusEquals
        | TokenKind::MinusEquals
        | TokenKind::DoubleAsteriskEquals
        | TokenKind::AsteriskEquals
        | TokenKind::SlashEquals
        | TokenKind::PercentEquals
        | TokenKind::ShiftLeftEquals
        | TokenKind::ShiftRightEquals
        | TokenKind::UnsignedShiftRightEquals
        | TokenKind::AmpersandEquals
        | TokenKind::CaretEquals
        | TokenKind::PipeEquals
        | TokenKind::DoubleAmpersandEquals
        | TokenKind::DoublePipeEquals
        | TokenKind::DoubleQuestionMarkEquals => 3,

        TokenKind::Yield => 2,

        TokenKind::Comma => 1,

        _ => unreachable!(),
    }
}

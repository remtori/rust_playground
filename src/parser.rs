pub mod lexer;
pub mod token;

use std::default::default;

use crate::ast::*;
use lexer::Lexer;
use token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser<'s> {
    lexer: Lexer<'s>,
    current_token: Token<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            lexer: Lexer::new(source),
            current_token: Token::default(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        self.consume();
        let mut program = Program::new();

        while !self.done() {
            if self.match_statement() {
                program.add_statement(self.parse_statement());
            } else {
                self.consume();
                unreachable!();
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Statement {
        if self.match_expression() {
            return Statement::ExpressionStatement(self.parse_expression(0, Associativity::Right));
        }

        unreachable!();
    }

    fn parse_expression(&mut self, min_precedence: u32, associativity: Associativity) -> Expression {
        let mut expr = self.parse_primary_expression();
        while self.match_secondary_expression() {
            let new_precedence = token_precedence(&self.current_token.kind());
            if (new_precedence < min_precedence)
            || (new_precedence == min_precedence && associativity == Associativity::Left) {
                break;
            }

            expr = self.parse_secondary_expression(
                expr,
                new_precedence,
                operator_associativity(&self.current_token.kind())
            );
        }

        expr
    }

    fn parse_primary_expression(&mut self) -> Expression {
        match self.current_token.kind() {
            TokenKind::ParenOpen => {
                self.consume_token(TokenKind::ParenOpen);
                let expr = self.parse_expression(0, default());
                self.consume_token(TokenKind::ParenClose);
                expr
            }
            TokenKind::BoolLiteral => Expression::Boolean(self.consume().bool_value()),
            TokenKind::NullLiteral => Expression::Null,
            TokenKind::StringLiteral => Expression::String(self.consume().string_value()),
            TokenKind::NumericLiteral => Expression::Numeric(self.consume().double_value()),
            _ => unreachable!(),
        }
    }

    fn parse_secondary_expression(&mut self, lhs: Expression, min_precedence: u32, associativity: Associativity) -> Expression {
        match self.current_token.kind() {
            TokenKind::Plus => {
                self.consume();
                Expression::Add(Box::new(lhs), Box::new(self.parse_expression(min_precedence, associativity)))
            }
            TokenKind::Minus => {
                self.consume();
                Expression::Sub(Box::new(lhs), Box::new(self.parse_expression(min_precedence, associativity)))
            }
            TokenKind::Asterisk => {
                self.consume();
                Expression::Mult(Box::new(lhs), Box::new(self.parse_expression(min_precedence, associativity)))
            }
            _ => unreachable!(),
        }
    }

    fn done(&self) -> bool {
        self.current_token.kind() == TokenKind::Eof
    }

    fn match_token(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind() == token_kind
    }

    fn match_statement(&self) -> bool {
        self.match_expression()
            || matches!(
                self.current_token.kind(),
                TokenKind::Return
                    | TokenKind::Do
                    | TokenKind::If
                    | TokenKind::Throw
                    | TokenKind::Try
                    | TokenKind::While
                    | TokenKind::With
                    | TokenKind::For
                    | TokenKind::CurlyOpen
                    | TokenKind::Switch
                    | TokenKind::Break
                    | TokenKind::Continue
                    | TokenKind::Var
                    | TokenKind::Debugger
                    | TokenKind::Semicolon
            )
    }

    fn match_expression(&self) -> bool {
        self.match_unary_prefixed_expression()
            || matches!(
                self.current_token.kind(),
                TokenKind::BoolLiteral
                    | TokenKind::NumericLiteral
                    | TokenKind::BigIntLiteral
                    | TokenKind::StringLiteral
                    | TokenKind::TemplateLiteralStart
                    | TokenKind::NullLiteral
                    | TokenKind::Identifier
                    | TokenKind::New
                    | TokenKind::CurlyOpen
                    | TokenKind::BracketOpen
                    | TokenKind::ParenOpen
                    | TokenKind::Function
                    | TokenKind::This
                    | TokenKind::Super
                    | TokenKind::RegexLiteral,
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

    fn consume_token(&mut self, token_kind: TokenKind) {
        if self.match_token(token_kind) {
            self.consume();
        } else {
            unreachable!();
        }
    }

    fn consume(&mut self) -> Token {
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
        | TokenKind::Comma
        => Associativity::Left,

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

        TokenKind::PlusPlus
        | TokenKind::MinusMinus => 18,

        TokenKind::ExclamationMark
        | TokenKind::Tilde
        | TokenKind::Typeof
        | TokenKind::Void
        | TokenKind::Delete
        | TokenKind::Await => 17,

        TokenKind::DoubleAsterisk => 16,

        TokenKind::Asterisk
        | TokenKind::Slash
        | TokenKind::Percent => 15,

        TokenKind::Plus
        | TokenKind::Minus => 14,

        TokenKind::ShiftLeft
        | TokenKind::ShiftRight
        | TokenKind::UnsignedShiftRight => 13,

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

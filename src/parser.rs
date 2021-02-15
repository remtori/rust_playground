pub mod lexer;
pub mod token;

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
            return Statement::ExpressionStatement(self.parse_expression());
        }

        unreachable!();
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expr = self.parse_primary_expression();
        while self.match_secondary_expression() {
            expr = self.parse_secondary_expression(expr);
        }

        expr
    }

    fn parse_primary_expression(&mut self) -> Expression {
        match self.current_token.kind() {
            TokenKind::ParenOpen => {
                self.consume_token(TokenKind::ParenOpen);
                let expr = self.parse_expression();
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

    fn parse_secondary_expression(&mut self, lhs: Expression) -> Expression {
        match self.current_token.kind() {
            TokenKind::Plus => {
                self.consume();
                Expression::Add(Box::new(lhs), Box::new(self.parse_expression()))
            }
            TokenKind::Minus => {
                self.consume();
                Expression::Sub(Box::new(lhs), Box::new(self.parse_expression()))
            }
            TokenKind::Asterisk => {
                self.consume();
                Expression::Mult(Box::new(lhs), Box::new(self.parse_expression()))
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
                | TokenKind::Instanceof
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

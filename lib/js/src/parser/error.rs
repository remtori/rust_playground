use std::fmt::Display;

use super::token::{Token, TokenKind};

#[derive(Debug)]
pub struct ParseError<'a> {
    kind: ErrorKind,
    token: Token<'a>,
}

#[derive(Debug)]
enum ErrorKind {
    Expect(TokenKind),
    Unexpected,
}

impl<'a> ParseError<'a> {
    pub fn expect(expect: TokenKind, token: Token<'a>) -> ParseError<'a> {
        ParseError {
            kind: ErrorKind::Expect(expect),
            token,
        }
    }

    pub fn unexpected(token: Token<'a>) -> ParseError<'a> {
        ParseError {
            kind: ErrorKind::Unexpected,
            token,
        }
    }
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SyntaxError at {}:{}: error={:?} token={:?}",
            self.token.line_number(),
            self.token.line_column(),
            self.kind,
            self.token,
        )?;

        Ok(())
    }
}

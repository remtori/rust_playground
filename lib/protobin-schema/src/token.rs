use std::str::FromStr;
use crate::error::{Error, ParseError, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Invalid,
    Eof,
    Message,
    Identifier,

    // Literal
    IntegerLiteral,

    // Punctuations
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    CurlyClose,
    CurlyOpen,
    ParenClose,
    ParenOpen,
    Semicolon,
    LessThan,
    GreaterThan,

    Ampersand,
    Asterisk,
    Caret,
    Equals,
    Minus,
    Plus,
    QuestionMark,
}

#[derive(Debug)]
pub struct Token<'s> {
    pub kind: TokenKind,
    pub value: &'s str,
    pub trivia: &'s str,
    pub doc_comment: &'s str,
    pub line: u32,
    pub column: u32,
}

impl<'s> Token<'s> {
    pub fn parse<T: FromStr>(&self) -> Result<T> {
        self.value.parse::<T>().map_err(|_| Error::Parse(ParseError {}))
    }
}

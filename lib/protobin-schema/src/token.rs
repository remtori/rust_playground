#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Invalid,
    Eof,
    Struct,
    Identifier,
    Comment,

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
    pub line: u32,
    pub column: u32,
}

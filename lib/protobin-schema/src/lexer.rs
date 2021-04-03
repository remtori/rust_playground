use std::collections::HashMap;

use crate::error::{Error, LexicalError, Result};
use crate::token::{Token, TokenKind};

pub struct Lexer<'s> {
    source: &'s str,
    position: usize,
    line: u32,
    column: u32,
    current_char: char,
    previous_token_kind: TokenKind,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Lexer<'s> {
        let mut lexer = Lexer {
            source,
            position: 0,
            line: 1,
            column: 0,
            current_char: '\0',
            previous_token_kind: TokenKind::Invalid,
        };

        lexer.consume().unwrap();

        lexer
    }

    pub fn next_token(&mut self) -> Result<Token<'s>> {
        let trivia_start = self.position - 1;

        // Consume white space + non-doc comment
        loop {
            if self.is_line_terminator() {
                self.consume()?;
                while !self.is_eof() && self.is_line_terminator() {
                    self.consume()?;
                }
            } else if self.current_char.is_whitespace() {
                self.consume()?;
                while !self.is_eof() && self.current_char.is_whitespace() {
                    self.consume()?;
                }
            } else if self.is_line_comment_start() {
                self.consume()?;
                while !self.is_eof() && !self.is_line_terminator() {
                    self.consume()?;
                }
            } else if self.is_block_comment_start() {
                self.consume()?;
                while !self.is_eof() && !self.is_block_comment_end() {
                    self.consume()?;
                }

                self.consume()?; // consume '*'
                self.consume()?; // consume '/'
            } else {
                break;
            }
        }

        let doc_start = self.position - 1;
        loop {
            if self.is_doc_comment_start() {
                self.consume()?;
                while !self.is_eof() && !self.is_line_terminator() {
                    self.consume()?;
                }
            } else if self.is_line_terminator() {
                self.consume()?;
                while !self.is_eof() && self.is_line_terminator() {
                    self.consume()?;
                }
            } else if self.current_char.is_whitespace() {
                self.consume()?;
                while !self.is_eof() && self.current_char.is_whitespace() {
                    self.consume()?;
                }
            } else {
                break;
            }
        }

        let value_start = self.position - 1;
        let value_start_line = self.line;
        let value_start_column = self.column;
        let kind = if self.is_identifier_start() {
            self.consume()?;
            while self.is_identifier_body() {
                self.consume()?;
            }

            if let Some(tk) = KEY_WORDS.get(&self.source[value_start..self.position - 1]) {
                *tk
            } else {
                TokenKind::Identifier
            }
        } else if self.current_char == '\0' {
            if self.previous_token_kind == TokenKind::Eof {
                return Err(Error::message("EOF"));
            }

            TokenKind::Eof
        } else if let Some(tk) = SINGLE_CHAR_TOKEN.get(&self.current_char) {
            self.consume()?;
            *tk
        } else {
            return Err(Error::Message(format!(
                "unknown char _{}_",
                self.current_char
            )));
        };

        self.previous_token_kind = kind;

        if trivia_start == value_start || value_start == self.position - 1 {
            Ok(Token {
                kind,
                trivia: "",
                value: "",
                doc_comment: "",
                line: value_start_line,
                column: value_start_column,
            })
        } else {
            Ok(Token {
                kind,
                trivia: &self.source[trivia_start..doc_start],
                doc_comment: &self.source[doc_start..value_start],
                value: &self.source[value_start..self.position - 1],
                line: value_start_line,
                column: value_start_column,
            })
        }
    }

    fn consume(&mut self) -> Result<()> {
        if self.position == self.source.len() {
            self.position += 1;
            self.column += 1;
            self.current_char = '\0';
            return Ok(());
        }

        if self.position > self.source.len() {
            return Err(Error::message("EOF"));
        }

        if self.is_line_terminator() {
            let is_second_char_of_crlf = self.position > 1
                && self.unchecked_peek_at_offset(-2) == '\r'
                && self.current_char == '\n';

            if !is_second_char_of_crlf {
                self.column = 1;
                self.line += 1;
            }
        } else {
            self.column += 1
        }

        self.current_char = self.unchecked_peek_at_offset(0);
        self.position += 1;
        Ok(())
    }

    fn is_identifier_start(&self) -> bool {
        self.current_char.is_alphabetic() || self.current_char == '$' || self.current_char == '_'
    }

    fn is_identifier_body(&self) -> bool {
        self.current_char.is_digit(10) || self.is_identifier_start()
    }

    fn is_line_comment_start(&self) -> bool {
        self.match_2('/', '/') && !self.is_doc_comment_start()
    }

    fn is_doc_comment_start(&self) -> bool {
        self.match_3('/', '/', '/')
    }

    fn is_block_comment_start(&self) -> bool {
        self.match_2('/', '*')
    }

    fn is_block_comment_end(&self) -> bool {
        self.match_2('*', '/')
    }

    fn is_line_terminator(&self) -> bool {
        matches!(
            self.current_char,
            '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}'
        )
    }

    fn is_eof(&self) -> bool {
        self.current_char == '\0' || self.position >= self.source.len()
    }

    fn match_2(&self, c1: char, c2: char) -> bool {
        self.position < self.source.len()
            && self.current_char == c1
            && self.unchecked_peek_at_offset(0) == c2
    }

    fn match_3(&self, c1: char, c2: char, c3: char) -> bool {
        self.position + 1 < self.source.len()
            && self.current_char == c1
            && self.unchecked_peek_at_offset(0) == c2
            && self.unchecked_peek_at_offset(1) == c3
    }

    fn unchecked_peek_at_offset(&self, offset: isize) -> char {
        self.source
            .chars()
            .nth((self.position as isize + offset) as usize)
            .unwrap()
    }
}

lazy_static::lazy_static! {
    static ref KEY_WORDS: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("message", TokenKind::Message);
        m
    };

    static ref SINGLE_CHAR_TOKEN: HashMap<char, TokenKind> = {
        let mut m = HashMap::new();
        m.insert('&', TokenKind::Ampersand);
        m.insert('*', TokenKind::Asterisk);
        m.insert('[', TokenKind::BracketOpen);
        m.insert(']', TokenKind::BracketClose);
        m.insert('^', TokenKind::Caret);
        m.insert('=', TokenKind::Equals);
        m.insert('-', TokenKind::Minus);
        m.insert('+', TokenKind::Plus);
        m.insert('?', TokenKind::QuestionMark);
        m.insert(':', TokenKind::Colon);
        m.insert(',', TokenKind::Comma);
        m.insert('{', TokenKind::CurlyOpen);
        m.insert('}', TokenKind::CurlyClose);
        m.insert('(', TokenKind::ParenOpen);
        m.insert(')', TokenKind::ParenClose);
        m.insert(';', TokenKind::Semicolon);
        m.insert('<', TokenKind::LessThan);
        m.insert('>', TokenKind::GreaterThan);
        m
    };
}

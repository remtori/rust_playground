use super::token::*;
use std::collections::HashMap;

const EOF: char = '\0';

#[derive(Debug)]
pub struct Lexer<'s> {
    source: &'s str,
    position: usize,
    previous_token_kind: TokenKind,
    current_char: char,
    line_number: usize,
    line_column: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = Lexer {
            source,
            position: 0,
            previous_token_kind: TokenKind::Eof,
            current_char: '\0',
            line_number: 1,
            line_column: 0,
        };

        lexer.consume();
        lexer
    }

    pub fn next_token(&mut self) -> Token<'s> {
        let trivia_start = self.position - 1;

        // consume whitespace and comments
        loop {
            if self.is_line_terminator() {
                loop {
                    self.consume();
                    if self.is_eof() || !self.is_line_terminator() {
                        break;
                    }
                }
            } else if self.current_char.is_whitespace() {
                loop {
                    self.consume();
                    if self.is_eof() || !self.current_char.is_whitespace() {
                        break;
                    }
                }
            } else if self.is_line_comment_start() {
                loop {
                    self.consume();
                    if self.is_eof() || self.is_line_terminator() {
                        break;
                    }
                }
            } else if self.is_block_comment_start() {
                loop {
                    self.consume();
                    if self.is_eof() || self.is_block_comment_end() {
                        break;
                    }
                }

                self.consume(); // consume '*'
                self.consume(); // consume '/'
            } else {
                break;
            }
        }

        let value_start = self.position - 1;
        let value_start_line_number = self.line_number;
        let value_start_line_column = self.line_column;
        let mut token_kind = TokenKind::Invalid;

        if self.is_identifier_start() {
            // Identifier or keywords

            loop {
                self.consume();
                if !self.is_identifier_body() {
                    break;
                }
            }

            if let Some(tk) = KEY_WORDS.get(&self.source[value_start..self.position - 1]) {
                token_kind = *tk;
            } else {
                token_kind = TokenKind::Identifier;
            }
        } else if self.is_numeric_literal_start() {
            token_kind = TokenKind::NumericLiteral;
            while self.current_char.is_digit(10) {
                self.consume();
            }
        } else if self.current_char == '\'' || self.current_char == '"' {
            let stop_char = self.current_char;
            loop {
                self.consume();

                if self.current_char == '\\'
                    && !self.is_eof()
                    && self.unchecked_peekato(0) == stop_char
                {
                    self.consume();
                    self.consume();
                }

                if self.current_char == stop_char {
                    self.consume();
                    break;
                }
            }
        } else if self.current_char == EOF {
            token_kind = TokenKind::Eof;
        } else {
            // The only four char operator: >>>=
            let mut found_4_char_token = false;
            if self.match_4('>', '>', '>', '=') {
                found_4_char_token = true;
                self.consume();
                self.consume();
                self.consume();
                self.consume();
            }

            let mut found_3_char_token = false;
            if let Some(tk) =
                THREE_CHAR_TOKEN.get(&self.source[self.position - 1..self.position + 2])
            {
                found_3_char_token = true;
                token_kind = *tk;
                self.consume();
                self.consume();
                self.consume();
            }

            let mut found_2_char_token = false;
            if let Some(tk) = TWO_CHAR_TOKEN.get(&self.source[self.position - 1..self.position + 1])
            {
                found_2_char_token = true;
                token_kind = *tk;
                self.consume();
                self.consume();
            }

            let mut found_single_char_token = false;
            if let Some(tk) = SINGLE_CHAR_TOKEN.get(&self.current_char) {
                found_single_char_token = true;
                token_kind = *tk;
                self.consume();
            }

            if !found_4_char_token
                && !found_3_char_token
                && !found_2_char_token
                && !found_single_char_token
            {
                self.consume();
                token_kind = TokenKind::Invalid;
            }
        }

        #[cfg(feature = "debug_lexer")]
        println!(
            "[Pos {} at {}:{}]: value={}:{} trivia={}:{}",
            self.position,
            self.line_number,
            self.line_column,
            value_start,
            self.position - 1,
            trivia_start,
            value_start
        );

        self.previous_token_kind = token_kind;

        Token::new(
            token_kind,
            &self.source[value_start..self.position - 1],
            &self.source[trivia_start..value_start],
            value_start_line_number,
            value_start_line_column,
        )
    }

    fn consume(&mut self) {
        #[cfg(feature = "debug_lexer")]
        println!(
            "[Pos {} at {}:{}]: Consumed: '{}' unicode=0x{:04x}",
            self.position,
            self.line_number,
            self.line_column,
            self.current_char,
            self.current_char as u32
        );

        if self.is_eof() {
            self.position += 1;
            self.line_column += 1;
            self.current_char = '\0';
            return;
        }

        if self.is_line_terminator() {
            let is_second_char_of_crlf = self.position > 1
                && self.unchecked_peekato(-2) == '\r'
                && self.current_char == '\n';

            if !is_second_char_of_crlf {
                self.line_column = 1;
                self.line_number += 1;
            }
        } else {
            self.line_column += 1
        }

        self.current_char = self.unchecked_peekato(0);
        self.position += 1
    }

    fn is_identifier_start(&self) -> bool {
        self.current_char.is_alphabetic() || self.current_char == '$' || self.current_char == '_'
    }

    fn is_identifier_body(&self) -> bool {
        self.current_char.is_digit(10) || self.is_identifier_start()
    }

    fn is_numeric_literal_start(&self) -> bool {
        self.current_char.is_digit(10)
            || (self.current_char == '.'
                && !self.is_eof()
                && self.unchecked_peekato(0).is_digit(10))
    }

    fn is_line_comment_start(&self) -> bool {
        self.match_2('/', '/')
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
        self.position >= self.source.len()
    }

    fn match_2(&self, c1: char, c2: char) -> bool {
        if self.position >= self.source.len() {
            false
        } else {
            self.current_char == c1 && self.unchecked_peekato(0) == c2
        }
    }

    fn match_3(&self, c1: char, c2: char, c3: char) -> bool {
        if self.position + 1 >= self.source.len() {
            false
        } else {
            self.current_char == c1
                && self.unchecked_peekato(0) == c2
                && self.unchecked_peekato(1) == c3
        }
    }

    fn match_4(&self, c1: char, c2: char, c3: char, c4: char) -> bool {
        if self.position + 2 >= self.source.len() {
            false
        } else {
            self.current_char == c1
                && self.unchecked_peekato(0) == c2
                && self.unchecked_peekato(1) == c3
                && self.unchecked_peekato(2) == c4
        }
    }

    fn unchecked_peekato(&self, offset: isize) -> char {
        self.source
            .chars()
            .nth((self.position as isize + offset) as usize)
            .unwrap()
    }
}

lazy_static! {
    static ref KEY_WORDS: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("await", TokenKind::Await);
        m.insert("break", TokenKind::Break);
        m.insert("case", TokenKind::Case);
        m.insert("catch", TokenKind::Catch);
        m.insert("class", TokenKind::Class);
        m.insert("const", TokenKind::Const);
        m.insert("continue", TokenKind::Continue);
        m.insert("debugger", TokenKind::Debugger);
        m.insert("default", TokenKind::Default);
        m.insert("delete", TokenKind::Delete);
        m.insert("do", TokenKind::Do);
        m.insert("else", TokenKind::Else);
        m.insert("enum", TokenKind::Enum);
        m.insert("export", TokenKind::Export);
        m.insert("extends", TokenKind::Extends);
        m.insert("false", TokenKind::BoolLiteral);
        m.insert("finally", TokenKind::Finally);
        m.insert("for", TokenKind::For);
        m.insert("function", TokenKind::Function);
        m.insert("if", TokenKind::If);
        m.insert("import", TokenKind::Import);
        m.insert("in", TokenKind::In);
        m.insert("instanceof", TokenKind::InstanceOf);
        m.insert("let", TokenKind::Let);
        m.insert("new", TokenKind::New);
        m.insert("null", TokenKind::NullLiteral);
        m.insert("return", TokenKind::Return);
        m.insert("super", TokenKind::Super);
        m.insert("switch", TokenKind::Switch);
        m.insert("this", TokenKind::This);
        m.insert("throw", TokenKind::Throw);
        m.insert("true", TokenKind::BoolLiteral);
        m.insert("try", TokenKind::Try);
        m.insert("typeof", TokenKind::Typeof);
        m.insert("var", TokenKind::Var);
        m.insert("void", TokenKind::Void);
        m.insert("while", TokenKind::While);
        m.insert("with", TokenKind::With);
        m.insert("yield", TokenKind::Yield);
        m
    };
    static ref THREE_CHAR_TOKEN: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("===", TokenKind::EqualsEqualsEquals);
        m.insert("!==", TokenKind::ExclamationMarkEqualsEquals);
        m.insert("**=", TokenKind::DoubleAsteriskEquals);
        m.insert("<<=", TokenKind::ShiftLeftEquals);
        m.insert(">>=", TokenKind::ShiftRightEquals);
        m.insert("&&=", TokenKind::DoubleAmpersandEquals);
        m.insert("||=", TokenKind::DoublePipeEquals);
        m.insert("??=", TokenKind::DoubleQuestionMarkEquals);
        m.insert(">>>", TokenKind::UnsignedShiftRight);
        m.insert("...", TokenKind::TripleDot);
        m
    };
    static ref TWO_CHAR_TOKEN: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("=>", TokenKind::Arrow);
        m.insert("+=", TokenKind::PlusEquals);
        m.insert("-=", TokenKind::MinusEquals);
        m.insert("*=", TokenKind::AsteriskEquals);
        m.insert("/=", TokenKind::SlashEquals);
        m.insert("%=", TokenKind::PercentEquals);
        m.insert("&=", TokenKind::AmpersandEquals);
        m.insert("|=", TokenKind::PipeEquals);
        m.insert("^=", TokenKind::CaretEquals);
        m.insert("&&", TokenKind::DoubleAmpersand);
        m.insert("||", TokenKind::DoublePipe);
        m.insert("??", TokenKind::DoubleQuestionMark);
        m.insert("**", TokenKind::DoubleAsterisk);
        m.insert("==", TokenKind::EqualsEquals);
        m.insert("<=", TokenKind::LessThanEquals);
        m.insert(">=", TokenKind::GreaterThanEquals);
        m.insert("!=", TokenKind::ExclamationMarkEquals);
        m.insert("--", TokenKind::MinusMinus);
        m.insert("++", TokenKind::PlusPlus);
        m.insert("<<", TokenKind::ShiftLeft);
        m.insert(">>", TokenKind::ShiftRight);
        m.insert("?.", TokenKind::QuestionMarkPeriod);
        m
    };
    static ref SINGLE_CHAR_TOKEN: HashMap<char, TokenKind> = {
        let mut m = HashMap::new();
        m.insert('&', TokenKind::Ampersand);
        m.insert('*', TokenKind::Asterisk);
        m.insert('[', TokenKind::BracketOpen);
        m.insert(']', TokenKind::BracketClose);
        m.insert('^', TokenKind::Caret);
        m.insert(':', TokenKind::Colon);
        m.insert(',', TokenKind::Comma);
        m.insert('{', TokenKind::CurlyOpen);
        m.insert('}', TokenKind::CurlyClose);
        m.insert('=', TokenKind::Equals);
        m.insert('!', TokenKind::ExclamationMark);
        m.insert('-', TokenKind::Minus);
        m.insert('(', TokenKind::ParenOpen);
        m.insert(')', TokenKind::ParenClose);
        m.insert('%', TokenKind::Percent);
        m.insert('.', TokenKind::Period);
        m.insert('|', TokenKind::Pipe);
        m.insert('+', TokenKind::Plus);
        m.insert('?', TokenKind::QuestionMark);
        m.insert(';', TokenKind::Semicolon);
        m.insert('/', TokenKind::Slash);
        m.insert('~', TokenKind::Tilde);
        m.insert('<', TokenKind::LessThan);
        m.insert('>', TokenKind::GreaterThan);
        m
    };
}

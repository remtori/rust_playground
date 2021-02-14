use super::token::*;
use std::collections::HashMap;

const EOF: char = '\0';

pub struct Lexer<'s> {
    source: &'s str,
    position: usize,
    current_token: Token<'s>,
    current_char: char,
    line_number: usize,
    line_column: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = Lexer {
            source,
            position: 0,
            current_token: Token::new(TokenType::Eof, "", "", 0, 0),
            current_char: '\0',
            line_number: 1,
            line_column: 0,
        };

        lexer.consume();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
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
        let mut token_type = TokenType::Invalid;

        if self.is_numeric_literal_start() {
            token_type = TokenType::NumericLiteral;
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
            token_type = TokenType::Eof;
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
                token_type = *tk;
                self.consume();
                self.consume();
                self.consume();
            }

            let mut found_2_char_token = false;
            if let Some(tk) = TWO_CHAR_TOKEN.get(&self.source[self.position - 1..self.position + 1])
            {
                found_2_char_token = true;
                token_type = *tk;
                self.consume();
                self.consume();
            }

            let mut found_single_char_token = false;
            if let Some(tk) = SINGLE_CHAR_TOKEN.get(&self.current_char) {
                found_single_char_token = true;
                token_type = *tk;
                self.consume();
            }

            if !found_4_char_token
                && !found_3_char_token
                && !found_2_char_token
                && !found_single_char_token
            {
                self.consume();
                token_type = TokenType::Invalid;
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

        self.current_token = Token::new(
            token_type,
            &self.source[value_start..self.position - 1],
            &self.source[trivia_start..value_start],
            value_start_line_number,
            value_start_line_column,
        );

        self.current_token
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
        matches!(self.current_char, '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}')
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
        self.source.chars().nth((self.position as isize + offset) as usize).unwrap()
    }
}

lazy_static! {
    static ref KEY_WORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("await", TokenType::Await);
        m.insert("break", TokenType::Break);
        m.insert("case", TokenType::Case);
        m.insert("catch", TokenType::Catch);
        m.insert("class", TokenType::Class);
        m.insert("const", TokenType::Const);
        m.insert("continue", TokenType::Continue);
        m.insert("debugger", TokenType::Debugger);
        m.insert("default", TokenType::Default);
        m.insert("delete", TokenType::Delete);
        m.insert("do", TokenType::Do);
        m.insert("else", TokenType::Else);
        m.insert("enum", TokenType::Enum);
        m.insert("export", TokenType::Export);
        m.insert("extends", TokenType::Extends);
        m.insert("false", TokenType::BoolLiteral);
        m.insert("finally", TokenType::Finally);
        m.insert("for", TokenType::For);
        m.insert("function", TokenType::Function);
        m.insert("if", TokenType::If);
        m.insert("import", TokenType::Import);
        m.insert("in", TokenType::In);
        m.insert("instanceof", TokenType::Instanceof);
        m.insert("let", TokenType::Let);
        m.insert("new", TokenType::New);
        m.insert("null", TokenType::NullLiteral);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("switch", TokenType::Switch);
        m.insert("this", TokenType::This);
        m.insert("throw", TokenType::Throw);
        m.insert("true", TokenType::BoolLiteral);
        m.insert("try", TokenType::Try);
        m.insert("typeof", TokenType::Typeof);
        m.insert("var", TokenType::Var);
        m.insert("void", TokenType::Void);
        m.insert("while", TokenType::While);
        m.insert("with", TokenType::With);
        m.insert("yield", TokenType::Yield);
        m
    };
    static ref THREE_CHAR_TOKEN: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("===", TokenType::EqualsEqualsEquals);
        m.insert("!==", TokenType::ExclamationMarkEqualsEquals);
        m.insert("**=", TokenType::DoubleAsteriskEquals);
        m.insert("<<=", TokenType::ShiftLeftEquals);
        m.insert(">>=", TokenType::ShiftRightEquals);
        m.insert("&&=", TokenType::DoubleAmpersandEquals);
        m.insert("||=", TokenType::DoublePipeEquals);
        m.insert("??=", TokenType::DoubleQuestionMarkEquals);
        m.insert(">>>", TokenType::UnsignedShiftRight);
        m.insert("...", TokenType::TripleDot);
        m
    };
    static ref TWO_CHAR_TOKEN: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("=>", TokenType::Arrow);
        m.insert("+=", TokenType::PlusEquals);
        m.insert("-=", TokenType::MinusEquals);
        m.insert("*=", TokenType::AsteriskEquals);
        m.insert("/=", TokenType::SlashEquals);
        m.insert("%=", TokenType::PercentEquals);
        m.insert("&=", TokenType::AmpersandEquals);
        m.insert("|=", TokenType::PipeEquals);
        m.insert("^=", TokenType::CaretEquals);
        m.insert("&&", TokenType::DoubleAmpersand);
        m.insert("||", TokenType::DoublePipe);
        m.insert("??", TokenType::DoubleQuestionMark);
        m.insert("**", TokenType::DoubleAsterisk);
        m.insert("==", TokenType::EqualsEquals);
        m.insert("<=", TokenType::LessThanEquals);
        m.insert(">=", TokenType::GreaterThanEquals);
        m.insert("!=", TokenType::ExclamationMarkEquals);
        m.insert("--", TokenType::MinusMinus);
        m.insert("++", TokenType::PlusPlus);
        m.insert("<<", TokenType::ShiftLeft);
        m.insert(">>", TokenType::ShiftRight);
        m.insert("?.", TokenType::QuestionMarkPeriod);
        m
    };
    static ref SINGLE_CHAR_TOKEN: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('&', TokenType::Ampersand);
        m.insert('*', TokenType::Asterisk);
        m.insert('[', TokenType::BracketOpen);
        m.insert(']', TokenType::BracketClose);
        m.insert('^', TokenType::Caret);
        m.insert(':', TokenType::Colon);
        m.insert(',', TokenType::Comma);
        m.insert('{', TokenType::CurlyOpen);
        m.insert('}', TokenType::CurlyClose);
        m.insert('=', TokenType::Equals);
        m.insert('!', TokenType::ExclamationMark);
        m.insert('-', TokenType::Minus);
        m.insert('(', TokenType::ParenOpen);
        m.insert(')', TokenType::ParenClose);
        m.insert('%', TokenType::Percent);
        m.insert('.', TokenType::Period);
        m.insert('|', TokenType::Pipe);
        m.insert('+', TokenType::Plus);
        m.insert('?', TokenType::QuestionMark);
        m.insert(';', TokenType::Semicolon);
        m.insert('/', TokenType::Slash);
        m.insert('~', TokenType::Tilde);
        m.insert('<', TokenType::LessThan);
        m.insert('>', TokenType::GreaterThan);
        m
    };
}

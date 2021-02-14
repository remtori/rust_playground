pub const NUMBER_OF_JS_TOKENS: usize = TokenType::NumberOfToken as usize;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum TokenCategory {
    Invalid,
    Number,
    String,
    Punctuation,
    Operator,
    Keyword,
    ControlKeyword,
    Identifier,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'p> {
    token_type: TokenType,
    value: &'p str,
    trivia: &'p str,
    line_number: usize,
    line_column: usize,
}

impl<'p> Token<'p> {
    pub fn new<'a>(
        token_type: TokenType,
        value: &'a str,
        trivia: &'a str,
        line_number: usize,
        line_column: usize,
    ) -> Token<'a> {
        Token { token_type, value, trivia, line_number, line_column }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn category(&self) -> TokenCategory {
        TOKEN_TYPE_TO_CATEGORY[self.token_type as usize]
    }

    pub fn value(&self) -> &str {
        self.value
    }

    pub fn trivia(&self) -> &str {
        self.trivia
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn line_column(&self) -> usize {
        self.line_column
    }

    pub fn double_value(&self) -> Option<f64> {
        assert_eq!(self.token_type, TokenType::NumericLiteral);
        self.value.parse::<f64>().ok()
    }

    pub fn bool_value(&self) -> bool {
        assert_eq!(self.token_type, TokenType::BoolLiteral);
        self.value.eq("true")
    }

    pub fn string_value(&self) -> String {
        String::from(self.value)
    }

    pub fn is_identifier_name(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Identifier
                | TokenType::Await
                | TokenType::BoolLiteral
                | TokenType::Break
                | TokenType::Case
                | TokenType::Catch
                | TokenType::Class
                | TokenType::Const
                | TokenType::Continue
                | TokenType::Default
                | TokenType::Delete
                | TokenType::Do
                | TokenType::Else
                | TokenType::Enum
                | TokenType::Export
                | TokenType::Extends
                | TokenType::Finally
                | TokenType::For
                | TokenType::Function
                | TokenType::If
                | TokenType::Import
                | TokenType::In
                | TokenType::Instanceof
                | TokenType::Interface
                | TokenType::Let
                | TokenType::New
                | TokenType::NullLiteral
                | TokenType::Return
                | TokenType::Super
                | TokenType::Switch
                | TokenType::This
                | TokenType::Throw
                | TokenType::Try
                | TokenType::Typeof
                | TokenType::Var
                | TokenType::Void
                | TokenType::While
                | TokenType::Yield
        )
    }
}

macro_rules! declare_tokens {
    ($(($type:tt, $category:tt));*) => {
        const TOKEN_TYPE_TO_CATEGORY: [TokenCategory; NUMBER_OF_JS_TOKENS] = [
            $(
                TokenCategory::$category,
            )*
        ];

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
        pub enum TokenType {
            $(
                $type,
            )*
            NumberOfToken
        }
    };
}

declare_tokens!(
    (Ampersand, Operator);
    (AmpersandEquals, Operator);
    (Arrow, Operator);
    (Asterisk, Operator);
    (AsteriskEquals, Operator);
    (Async, Keyword);
    (Await, Keyword);
    (BigIntLiteral, Number);
    (BoolLiteral, Keyword);
    (BracketClose, Punctuation);
    (BracketOpen, Punctuation);
    (Break, Keyword);
    (Caret, Operator);
    (CaretEquals, Operator);
    (Case, ControlKeyword);
    (Catch, ControlKeyword);
    (Class, Keyword);
    (Colon, Punctuation);
    (Comma, Punctuation);
    (Const, Keyword);
    (Continue, ControlKeyword);
    (CurlyClose, Punctuation);
    (CurlyOpen, Punctuation);
    (Debugger, Keyword);
    (Default, ControlKeyword);
    (Delete, Keyword);
    (Do, ControlKeyword);
    (DoubleAmpersand, Operator);
    (DoubleAmpersandEquals, Operator);
    (DoubleAsterisk, Operator);
    (DoubleAsteriskEquals, Operator);
    (DoublePipe, Operator);
    (DoublePipeEquals, Operator);
    (DoubleQuestionMark, Operator);
    (DoubleQuestionMarkEquals, Operator);
    (Else, ControlKeyword);
    (Enum, Keyword);
    (Eof, Invalid);
    (Equals, Operator);
    (EqualsEquals, Operator);
    (EqualsEqualsEquals, Operator);
    (ExclamationMark, Operator);
    (ExclamationMarkEquals, Operator);
    (ExclamationMarkEqualsEquals, Operator);
    (Export, Keyword);
    (Extends, Keyword);
    (Finally, ControlKeyword);
    (For, ControlKeyword);
    (Function, Keyword);
    (GreaterThan, Operator);
    (GreaterThanEquals, Operator);
    (Identifier, Identifier);
    (If, ControlKeyword);
    (Implements, Keyword);
    (Import, Keyword);
    (In, Keyword);
    (Instanceof, Keyword);
    (Interface, Keyword);
    (Invalid, Invalid);
    (LessThan, Operator);
    (LessThanEquals, Operator);
    (Let, Keyword);
    (Minus, Operator);
    (MinusEquals, Operator);
    (MinusMinus, Operator);
    (New, Keyword);
    (NullLiteral, Keyword);
    (NumericLiteral, Number);
    (Package, Keyword);
    (ParenClose, Punctuation);
    (ParenOpen, Punctuation);
    (Percent, Operator);
    (PercentEquals, Operator);
    (Period, Operator);
    (Pipe, Operator);
    (PipeEquals, Operator);
    (Plus, Operator);
    (PlusEquals, Operator);
    (PlusPlus, Operator);
    (Private, Keyword);
    (Protected, Keyword);
    (Public, Keyword);
    (QuestionMark, Operator);
    (QuestionMarkPeriod, Operator);
    (RegexFlags, String);
    (RegexLiteral, String);
    (Return, ControlKeyword);
    (Semicolon, Punctuation);
    (ShiftLeft, Operator);
    (ShiftLeftEquals, Operator);
    (ShiftRight, Operator);
    (ShiftRightEquals, Operator);
    (Slash, Operator);
    (SlashEquals, Operator);
    (Static, Keyword);
    (StringLiteral, String);
    (Super, Keyword);
    (Switch, ControlKeyword);
    (TemplateLiteralEnd, String);
    (TemplateLiteralExprEnd, Punctuation);
    (TemplateLiteralExprStart, Punctuation);
    (TemplateLiteralStart, String);
    (TemplateLiteralString, String);
    (This, Keyword);
    (Throw, ControlKeyword);
    (Tilde, Operator);
    (TripleDot, Operator);
    (Try, ControlKeyword);
    (Typeof, Keyword);
    (UnsignedShiftRight, Operator);
    (UnsignedShiftRightEquals, Operator);
    (UnterminatedRegexLiteral, String);
    (UnterminatedStringLiteral, String);
    (UnterminatedTemplateLiteral, String);
    (Var, Keyword);
    (Void, Keyword);
    (While, ControlKeyword);
    (With, ControlKeyword);
    (Yield, ControlKeyword)
);

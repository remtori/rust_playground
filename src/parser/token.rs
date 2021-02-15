#[derive(Debug)]
pub struct Token<'l> {
    kind: TokenKind,
    value: &'l str,
    trivia: &'l str,
    line_number: usize,
    line_column: usize,
}

impl<'l> Token<'l> {
    pub fn new<'a>(
        kind: TokenKind,
        value: &'a str,
        trivia: &'a str,
        line_number: usize,
        line_column: usize,
    ) -> Token<'a> {
        Token { kind, value, trivia, line_number, line_column }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn double_value(&self) -> Option<f64> {
        assert_eq!(self.kind, TokenKind::NumericLiteral);
        self.value.parse::<f64>().ok()
    }

    pub fn bool_value(&self) -> bool {
        assert_eq!(self.kind, TokenKind::BoolLiteral);
        self.value.eq("true")
    }

    pub fn string_value(&self) -> String {
        String::from(self.value)
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

    pub fn is_identifier_name(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Identifier
                | TokenKind::Await
                | TokenKind::BoolLiteral
                | TokenKind::Break
                | TokenKind::Case
                | TokenKind::Catch
                | TokenKind::Class
                | TokenKind::Const
                | TokenKind::Continue
                | TokenKind::Default
                | TokenKind::Delete
                | TokenKind::Do
                | TokenKind::Else
                | TokenKind::Enum
                | TokenKind::Export
                | TokenKind::Extends
                | TokenKind::Finally
                | TokenKind::For
                | TokenKind::Function
                | TokenKind::If
                | TokenKind::Import
                | TokenKind::In
                | TokenKind::Instanceof
                | TokenKind::Interface
                | TokenKind::Let
                | TokenKind::New
                | TokenKind::NullLiteral
                | TokenKind::Return
                | TokenKind::Super
                | TokenKind::Switch
                | TokenKind::This
                | TokenKind::Throw
                | TokenKind::Try
                | TokenKind::Typeof
                | TokenKind::Var
                | TokenKind::Void
                | TokenKind::While
                | TokenKind::Yield
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
// #[repr(u8)]
pub enum TokenKind {
    Invalid,
    Eof,

    // Keywords
    Async,
    Await,
    BoolLiteral,
    Break,
    Class,
    Const,
    Debugger,
    Delete,
    Enum,
    Export,
    Extends,
    Function,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Let,
    New,
    NullLiteral,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Super,
    This,
    Typeof,
    Var,
    Void,

    // Control Keywords
    Case,
    Catch,
    Continue,
    Default,
    Do,
    Else,
    Finally,
    For,
    If,
    Return,
    Switch,
    Throw,
    Try,
    While,
    With,
    Yield,

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
    TemplateLiteralExprEnd,
    TemplateLiteralExprStart,

    // Operators
    Ampersand,
    AmpersandEquals,
    Arrow,
    Asterisk,
    AsteriskEquals,
    Caret,
    CaretEquals,
    DoubleAmpersand,
    DoubleAmpersandEquals,
    DoubleAsterisk,
    DoubleAsteriskEquals,
    DoublePipe,
    DoublePipeEquals,
    DoubleQuestionMark,
    DoubleQuestionMarkEquals,
    Equals,
    EqualsEquals,
    EqualsEqualsEquals,
    ExclamationMark,
    ExclamationMarkEquals,
    ExclamationMarkEqualsEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Minus,
    MinusEquals,
    MinusMinus,
    Percent,
    PercentEquals,
    Period,
    Pipe,
    PipeEquals,
    Plus,
    PlusEquals,
    PlusPlus,
    QuestionMark,
    QuestionMarkPeriod,
    ShiftLeft,
    ShiftLeftEquals,
    ShiftRight,
    ShiftRightEquals,
    Slash,
    SlashEquals,
    Tilde,
    TripleDot,
    UnsignedShiftRight,
    UnsignedShiftRightEquals,

    Identifier,

    BigIntLiteral,
    NumericLiteral,

    RegexFlags,
    RegexLiteral,
    StringLiteral,
    TemplateLiteralEnd,
    TemplateLiteralStart,
    TemplateLiteralString,
    UnterminatedRegexLiteral,
    UnterminatedStringLiteral,
    UnterminatedTemplateLiteral,
}

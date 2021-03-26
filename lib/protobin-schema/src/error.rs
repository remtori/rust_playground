pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Lexical(LexicalError),
    Message(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Parse(err) => write!(f, "ParseError: {:?}", err),
            Error::Lexical(err) => write!(f, "LexicalError: {:?}", err),
            Error::Message(err) => f.write_str(err),
        }
    }
}

impl std::error::Error for Error {}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<LexicalError> for Error {
    fn from(e: LexicalError) -> Self {
        Error::Lexical(e)
    }
}

impl Error {
    pub fn message(msg: &str) -> Error {
        Error::Message(msg.to_owned())
    }
}

#[derive(Debug)]
pub struct ParseError {}

#[derive(Debug)]
pub struct LexicalError {}

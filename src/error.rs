
use std::fmt;
use std::error;

#[derive(Debug, PartialEq)]
/// Kinds of nccl errors.
pub enum ErrorKind {
    KeyNotFound,
    IndentationError,
    NameError,
    NoValue,
    MultipleValues,
    ParseError,
    FromStrError,
    FileError,
}

#[derive(Debug, PartialEq)]
/// nccl error type.
pub struct NcclError {
    kind: ErrorKind,
    line: u64,
    message: String,
}

impl NcclError {
    /// Creates a new NcclError.
    pub fn new(kind: ErrorKind, message: &str, line: u64) -> Self {
        NcclError {
            kind: kind,
            message: message.to_owned(),
            line: line,
        }
    }
}

impl fmt::Display for NcclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::ParseError | ErrorKind::IndentationError
                => write!(f, "An error has ocurred: {:?} on line {}\n\t{}", self.kind, self.line, self.message),
            _ => write!(f, "An error has ocurred: {:?}\n\t{}", self.kind, self.message)
        }
    }
}

impl error::Error for NcclError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::ParseError | ErrorKind::IndentationError
                => "An error has occurred while parsing.",
            _ => "An error has occurred while interacting with a Pair"
        }
    }
}


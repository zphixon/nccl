
use std::fmt;

#[derive(Debug, PartialEq)]
/// nccl Error type.
pub enum ErrorKind {
    KeyNotFound,
    IndentationError,
    NameError,
    NoValue,
    ParseError,
    FileError,
}

#[derive(Debug, PartialEq)]
pub struct NcclError {
    kind: ErrorKind,
    line: u64,
    message: String,
}

impl NcclError {
    pub fn new(kind: ErrorKind, message: &str, line: u64) -> Self {
        let r = NcclError {
            kind: kind,
            message: message.to_owned(),
            line: line,
        };
        println!("{}", &r);
        r
    }
}

impl fmt::Display for NcclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has ocurred: {:?} on line {}\n\t{}", self.kind, self.line, self.message)
    }
}


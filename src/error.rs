
use std::fmt;

#[derive(Debug)]
/// nccl Error type.
pub enum ErrorKind {
    KeyNotFound,
    IndentationError,
    NameError,
    NoValue,
    ParseError,
    FileError,
}

#[derive(Debug)]
pub struct NcclError {
    kind: ErrorKind,
    line: u64,
    message: String,
}

impl NcclError {
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
        write!(f, "An error has ocurred: {:?} on line {}\n\t{}", self.kind, self.line, self.message)
    }
}

//impl ::std::fmt::Display for ErrorKind {
//    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//        write!(f, "An error has ocurred: {:?}", self)
//    }
//}
//
//impl ::std::error::Error for Error {
//    fn description(&self) -> &str {
//        match *self {
//            Error::KeyNotFound => "Key not found.",
//            Error::IndentationError => "Incorrect indentation: Not 4 spaces.",
//            Error::NameError => "Schema not found.",
//            Error::NoValue => "No value associated with key.",
//            Error::ParseError => "Unable to parse value.",
//            Error::FileError => "Could not open file.",
//        }
//    }
//}


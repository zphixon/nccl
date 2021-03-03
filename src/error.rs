use std::fmt;

#[derive(Debug, PartialEq, Clone)]
/// Kinds of nccl errors.
pub enum ErrorKind {
    KeyNotFound,
    Indentation,
    Into,
    Name,
    NoValue,
    MultipleValues,
    Parse,
    FromStr,
    File,
    Utf8 { err: std::string::FromUtf8Error },
    Io,
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
            message: match kind {
                ErrorKind::Parse | ErrorKind::Indentation => format!(
                    "An error has ocurred: {:?} on line {}\n\t{}",
                    kind, line, message
                ),
                _ => format!("An error has ocurred: {:?}\n\t{}", kind, message),
            },
            kind,
            line,
        }
    }
}

impl fmt::Display for NcclError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Parse | ErrorKind::Indentation => write!(
                f,
                "An error has ocurred: {:?} on line {}\n\t{}",
                self.kind, self.line, self.message
            ),
            _ => write!(
                f,
                "An error has ocurred: {:?}\n\t{}",
                self.kind, self.message
            ),
        }
    }
}

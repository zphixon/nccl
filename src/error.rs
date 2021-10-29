use crate::token::{Span, TokenKind};

use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
/// Errors that may occur while parsing
pub enum NcclError {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The location of the token.
        span: Span,
        /// The kind of token we expected.
        expected: TokenKind,
        /// The kind of token we got.
        got: TokenKind,
    },
    /// The string was not terminated before the end of the file.
    UnterminatedString {
        /// The line the string starts on.
        start: usize,
    },
    /// There were non-comment characters after a quoted string.
    TrailingCharacters {
        /// The line the string ends on.
        line: usize,
    },
    /// The escape code in the file was unknown.
    ScanUnknownEscape {
        /// The line of the code.
        line: usize,
        /// The column of the code.
        column: usize,
        /// The code itself.
        escape: char,
    },
    /// The escape literal in the key was unknown. See [`crate::config::Config::parse_quoted`].
    ParseUnknownEscape {
        /// The escape code.
        escape: char,
    },
    /// A utf-8 string could not be constructed.
    Utf8 {
        /// The error.
        err: Utf8Error,
    },
}

impl std::fmt::Display for NcclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NcclError::UnexpectedToken {
                span,
                expected,
                got,
            } => write!(
                f,
                "expected {:?}, got {:?} at {}:{}",
                expected, got, span.line, span.column,
            ),
            NcclError::UnterminatedString { start } => {
                write!(f, "unterminated string starting on line {}", start)
            }
            NcclError::TrailingCharacters { line } => {
                write!(f, "characters after string on line {}", line)
            }
            NcclError::ScanUnknownEscape {
                escape,
                line,
                column,
            } => write!(f, "unknown escape {:?} at {}:{}", escape, line, column),
            NcclError::ParseUnknownEscape { escape } => write!(f, "unknown escape {:?}", escape),
            NcclError::Utf8 { err } => write!(f, "{}", err),
        }
    }
}

impl From<Utf8Error> for NcclError {
    fn from(err: Utf8Error) -> Self {
        NcclError::Utf8 { err }
    }
}

impl From<FromUtf8Error> for NcclError {
    fn from(err: FromUtf8Error) -> Self {
        NcclError::Utf8 {
            err: err.utf8_error(),
        }
    }
}

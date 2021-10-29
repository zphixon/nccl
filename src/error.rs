use crate::token::{Span, TokenKind};
use std::fmt::Formatter;

use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
/// Kinds of nccl errors.
pub enum NcclError {
    UnexpectedToken {
        span: Span,
        expected: TokenKind,
        got: TokenKind,
    },
    UnterminatedString {
        start: usize,
    },
    TrailingCharacters {
        line: usize,
    },
    ScanUnknownEscape {
        line: usize,
        column: usize,
        escape: char,
    },
    ParseUnknownEscape {
        escape: char,
    },
    Utf8 {
        err: Utf8Error,
    },
}

impl std::fmt::Display for NcclError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

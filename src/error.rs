
#[derive(Debug)]
/// nccl Error type.
pub enum Error {
    KeyNotFound,
    IndentationError,
    NameError,
    NoValue,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "An error has ocurred: {:?}", self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::KeyNotFound => "Key not found.",
            Error::IndentationError => "Incorrect indentation: Not 4 spaces.",
            Error::NameError => "Schema not found.",
            Error::NoValue => "No value associated with key.",
        }
    }
}


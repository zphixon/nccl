
//! Nccl is an easy way to add minimal configuration to your crate without
//! having to deal with complicated interfaces, obnoxious syntax, or outdated
//! languages. Nccl makes it easy for a user to pick up your configuration.
//! It's as easy as five cents.
//!
//! Nccl was motivated by the fact that other configuration languages are too
//! complicated for end-users. Strict enforcement of data types is a hassle for
//! people who just want stuff to do things. In nccl's case, not having data
//! types is motivated by the fact that a lot of configuration isn't based on
//! numbers, dates, or booleans, but strings. In the case where a more
//! complicated format is required, it's as simple as a call to `.value_as()` or
//! `.keys_as()`.

mod error;
mod pair;
mod parser;
mod token;
mod scanner;
mod value;
mod macros;

pub use error::*;
pub use pair::*;
pub use value::*;
pub use macros::*;

use parser::*;
use scanner::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

/// Parses a file using the given filename.
///
/// Examples:
///
/// ```
/// let config = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = config["server"]["port"].keys_as::<i64>().unwrap();
/// assert_eq!(ports, vec![80, 443]);
/// ```
pub fn parse_file(filename: &str) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new(Scanner::new(data).scan_tokens()?).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}

/// Parses a file, merging the results with the supplied pair. Allows for a
/// kind of inheritance of configuration.
///
/// Examples:
///
/// ```
/// let schemas = nccl::parse_file("examples/inherit.nccl").unwrap();
/// let user = nccl::parse_file_with("examples/inherit2.nccl", schemas).unwrap();
/// assert_eq!(user["sandwich"]["meat"].keys_as::<String>().unwrap().len(), 3);
/// assert_eq!(user["hello"]["world"].keys_as::<String>().unwrap().len(), 3);
/// ```
pub fn parse_file_with(filename: &str, pair: Pair) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new_with(Scanner::new(data).scan_tokens()?, pair).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}

/// Parses raw string data.
///
/// Examples:
///
/// ```
/// let raw = nccl::parse_string("hello\n\tworld!").unwrap();
/// assert_eq!(raw["hello"].value_as::<String>().unwrap(), "world!");
/// ```
pub fn parse_string(data: &str) -> Result<Pair, Vec<Box<Error>>> {
    Parser::new(Scanner::new(data.to_owned()).scan_tokens()?).parse()
}

/// Allows safe type conversions. Copied from nightly stdlib.
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

/// Allows safe type conversions. Copied from nightly stdlib.
pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

/// Allows safe type conversions. Copied from nightly stdlib.
impl<T, U> TryInto<U> for T where U: TryFrom<T> {
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}


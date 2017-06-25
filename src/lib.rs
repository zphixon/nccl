
mod error;
mod pair;
mod parser;
mod token;
mod scanner;

pub use error::*;
pub use pair::*;

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
/// let ports = config["server"]["port"].keys_as::<i32>().unwrap();
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
/// assert_eq!(user["sandwich"]["meat"].keys().len(), 3);
/// assert_eq!(user["hello"]["world"].keys().len(), 3);
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
/// assert_eq!(raw["hello"].value().unwrap(), "world!");
/// ```
pub fn parse_string(data: &str) -> Result<Pair, Vec<Box<Error>>> {
    Parser::new(Scanner::new(data.to_owned()).scan_tokens()?).parse()
}


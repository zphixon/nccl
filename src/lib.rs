
mod error;
mod pair;
mod parser;
mod token;
mod scanner;

pub use error::*;
pub use pair::*;
pub use parser::*;
pub use token::*;
pub use scanner::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn parse_file(filename: &str) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new(Scanner::new(data).scan_tokens()?).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}

pub fn parse_string(data: &str) -> Result<Pair, Vec<Box<Error>>> {
    Parser::new(Scanner::new(data.to_owned()).scan_tokens()?).parse()
}

pub fn parse_file_with(filename: &str, pair: Pair) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new_with(Scanner::new(data).scan_tokens()?, pair).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}


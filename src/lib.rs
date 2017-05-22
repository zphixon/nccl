
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

pub fn parse_file(filename: &str) -> Result<Vec<Token>, NcclError> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Scanner::new(data).scan_tokens()
    } else {
        Err(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))
    }
}


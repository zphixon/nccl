
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

pub fn parse_file(filename: &str) -> Result<Pair, Vec<NcclError>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        //let tokens = Scanner::new(data).scan_tokens()?;
        Parser::new(Scanner::new(data).scan_tokens()?).parse()
    } else {
        Err(vec![NcclError::new(ErrorKind::FileError, "Could not find file.", 0)])
    }
}

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        match token.kind {
            TokenKind::Value => print!("\"{}\"", token.lexeme),
            TokenKind::Colon => print!(" : "),
            TokenKind::Indent => print!(" >> "),
            TokenKind::Newline => println!(""),
            TokenKind::EOF => println!("end"),
        }
    }
}


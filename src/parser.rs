
use std::fs::File;
use std::io::Read;
use std::path::Path;

use pair::Pair;
use error::Error;

#[allow(dead_code)]
pub struct Parser {
    position: usize,
    line: u64,
    column: u64,
    pairs: Pair,
    data: Vec<u8>,
}

impl Parser {
    pub fn new(data: String) -> Self {
        Parser {
            position: 0,
            line: 0,
            column: 0,
            pairs: Pair::new("((top_level))"),
            data: data.into_bytes(),
        }
    }

    pub fn parse(&mut self) -> Result<Pair, Error> {
        Err(Error::ParseError)
    }

    fn match_tokens(&mut self, chars: Vec<u8>) -> bool {
        for c in chars {
            if self.check(c) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token: u8) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek() == token
        }
    }

    fn peek(&mut self) -> u8 {
        self.data[self.position]
    }

    fn advance(&mut self) {}

    fn is_at_end(&mut self) -> bool {
        false
    }
}

pub fn parse_file(filename: &str) -> Result<Pair, Error> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let mut parser = Parser::new(data);
        parser.parse()
    } else {
        Err(Error::FileError)
    }
}


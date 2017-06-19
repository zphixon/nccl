
use pair::Pair;
use error::{NcclError, ErrorKind};
use token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    current: usize,
    path: Vec<String>,
    indent: usize,
    tokens: Vec<Token>,
    pair: Pair,
    line: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            path: vec![],
            indent: 0,
            tokens: tokens,
            pair: Pair::new("__top_level__"),
            line: 1,
        }
    }

    // faked you out with that Scanner, didn't I?
    // you thought this was going to be recursive descent. YOU WERE WRONG!
    pub fn parse(mut self) -> Result<Pair, Vec<NcclError>> {
        let mut errors = vec![];
        let mut i = 0;

        while i < self.tokens.len() {
            match self.tokens[i].kind {
                TokenKind::Value => { // add to path respective of self.index
                    if self.indent <= self.path.len() {
                        let mut new = self.path[0..self.indent].to_owned();
                        new.push(self.tokens[i].lexeme.clone());
                        self.path = new;
                    } else {
                        self.path.push(self.tokens[i].lexeme.clone());
                    }

                    self.pair.add_slice(&self.path);

                    if i + 2 <= self.tokens.len() && self.tokens[i + 2].kind == TokenKind::Value {
                        self.path.clear();
                        self.indent = 0;
                    }
                    println!("{} : {:?}", self.tokens[i].lexeme, self.path);
                },

                TokenKind::Colon => { // clone existing w/ schema name, rename
                    println!("{:?}", &self.tokens[i - 1 .. i + 5]);
                    //let mut p = self.pair.clone();
                    //if let Ok(mut x) = p.get(&self.tokens[i + 1].lexeme) {
                    //    x.key = self.tokens[i - 1].lexeme.clone();
                    //    //panic!("{:?}", &self.tokens[i - 3 .. i + 2]);
                    //} else {
                    //    errors.push(NcclError::new(ErrorKind::ParseError, &format!("Schema not found: {}", self.tokens[i + 1].lexeme), self.line));
                    //}
                    i += 2;
                    self.indent += 1;
                },

                TokenKind::Indent => { // set new self.index
                    let mut indent = 0;

                    while self.tokens[i].kind == TokenKind::Indent {
                        indent += 1;
                        i += 1;
                    }

                    i -= 1;

                    self.indent = indent;
                },

                TokenKind::Newline => { // reset self.index
                    self.indent = 0;
                    self.line += 1;
                },

                TokenKind::EOF => break,
            }
            i += 1;
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(self.pair)
        }
    }
}


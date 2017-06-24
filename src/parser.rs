
use pair::Pair;
use token::{Token, TokenKind};
use error::{NcclError, ErrorKind};

use std::error::Error;

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

    pub fn new_with(tokens: Vec<Token>, pair: Pair) -> Self {
        Parser {
            current: 0,
            path: vec![],
            indent: 0,
            tokens: tokens,
            pair: pair,
            line: 1
        }
    }

    // faked you out with that Scanner, didn't I?
    // you thought this was going to be recursive descent. YOU WERE WRONG!
    pub fn parse(mut self) -> Result<Pair, Vec<Box<Error>>> {
        let mut errors: Vec<Box<Error>> = vec![];
        let mut prev_indent = 0;
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
                },

                TokenKind::Indent => { // set new self.index
                    let mut indent = 0;

                    while self.tokens[i].kind == TokenKind::Indent {
                        indent += 1;
                        i += 1;
                    }

                    i -= 1;

                    if prev_indent > indent {
                        if prev_indent - indent == 1 || prev_indent - indent == 0 {
                            self.indent = indent;
                        } else {
                            errors.push(Box::new(NcclError::new(ErrorKind::IndentationError, "Incorrect level of indentation found", self.line)));
                            self.indent = prev_indent;
                        }
                    } else {
                        if indent - prev_indent == 1 || indent - prev_indent == 0 {
                            self.indent = indent;
                        } else {
                            errors.push(Box::new(NcclError::new(ErrorKind::IndentationError, "Incorrect level of indentation found", self.line)));
                            self.indent = prev_indent;
                        }
                    }
                },

                TokenKind::Newline => { // reset self.index
                    prev_indent = self.indent;
                    self.indent = 0;
                    self.line += 1;
                },

                TokenKind::EOF => break,
            }
            i += 1;
        }

        if errors.len() == 0 {
            Ok(self.pair)
        } else {
            Err(errors)
        }
    }
}


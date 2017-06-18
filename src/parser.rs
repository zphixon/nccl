
use pair::Pair;
use error::{NcclError, ErrorKind};
use token::{Token, TokenKind};

use std::io::Write;

pub struct Parser {
    current: usize,
    path: Vec<String>,
    tokens: Vec<Token>,
    pair: Pair,
    line: u64,
}

// nccl = value+
// value = name schema? nl key*
// key = indent (name nl | indent value)
// name = not colon
// nl = newline
// schema = ":" name

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            path: vec![],
            tokens: tokens,
            pair: Pair::new("__top_level__"),
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Pair, NcclError> {
        while !self.is_at_end() {
            self.value()?;
            self.advance();
        }

        Ok(self.pair.clone())
    }

    fn add(&mut self, value: String) {
        //self.pair.add_vec(self.path, value);
    }

    fn value(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.name()?;

            if self.peek().kind == TokenKind::Colon {
                self.schema()?;
            }

            self.newline()?;

            while self.peek().kind != TokenKind::Indent {
                self.key()?;
            }

            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected value, found EOF", self.line))
        }
    }

    fn key(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.indent()?;

            if self.peek().kind == TokenKind::Value {
                self.name()?;
                self.newline()?;
            } else if self.peek().kind == TokenKind::Indent {
                self.indent()?;
                self.value()?;
            }

            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected key, found EOF", self.line))
        }
    }

    fn schema(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.colon()?;
            self.name()?;
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected schema, found EOF", self.line))
        }
    }

    fn name(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.advance();
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected name, found EOF", self.line))
        }
    }

    fn indent(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.advance();
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected indent, found EOF", self.line))
        }
    }

    fn newline(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.line += 1;
            self.advance();
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected newline, found EOF", self.line))
        }
    }

    fn colon(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            self.advance();
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected colon, found EOF", self.line))
        }
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, NcclError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, message, self.line))
        }
    }
}


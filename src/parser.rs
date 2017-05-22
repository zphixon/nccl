
use pair::Pair;
use error::{NcclError, ErrorKind};
use token::{Token, TokenKind};

#[allow(dead_code)]
pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
    pair: Pair,
    line: u64,
}

// nccl = (value (":" value)? newline indent value newline)*
// value = [^:]+
// newline = "\n" | "\r\n"
// indent = " "+ | "\t"

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens: tokens,
            pair: Pair::new("__top_level__"),
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Pair, NcclError> {
        Ok(self.pair.clone())
    }

    fn add(&mut self, value: String) {
        self.pair.add(&value);
    }

    fn nccl(&mut self) -> Result<(), NcclError> {
        while !self.is_at_end() {
            self.value()?;
            if self.peek().kind == TokenKind::Colon {
                self.colon()?;
                self.schema()?;
            }
            self.newline()?;
            self.indent()?;
            self.value()?;
            self.newline()?;
        }
        Ok(())
    }

    fn value(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            if self.peek().kind != TokenKind::Value {
            } else {
                return Err(NcclError::new(ErrorKind::ParseError, &format!("Expected value, found {:?}", self.peek().kind), self.line));
            }
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected value, found EOF", self.line))
        }
    }

    fn indent(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected indent, found EOF", self.line))
        }
    }

    fn newline(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected newline, found EOF", self.line))
        }
    }

    fn schema(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
            Ok(())
        } else {
            Err(NcclError::new(ErrorKind::ParseError, "Expected schema, found EOF", self.line))
        }
    }

    fn colon(&mut self) -> Result<(), NcclError> {
        if !self.is_at_end() {
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


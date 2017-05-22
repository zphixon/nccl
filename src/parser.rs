
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

// nccl = (value (":" value)? indent value)*
// value = [^:]+
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
        Ok(Pair::new(""))
    }

    fn nccl(&mut self) {}
    fn value(&mut self) {}
    fn indent(&mut self) {}

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


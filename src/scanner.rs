
use token::{Token, TokenKind};
use error::Error;

// ranked worst to least
enum Indent {
    Neither,
    Tabs,
    Spaces(u8),
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
    indent: Indent,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.into_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            indent: Indent::Neither,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
        }

        self.tokens.push(Token::new(TokenKind::EOF, "".into(), self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        match self.advance() {
            b':' => {
                self.add_token(TokenKind::Colon);
                while self.peek() == b' ' && !self.is_at_end() {
                    self.advance();
                }
                if self.is_at_end() {
                    return Err(Error::ParseError);
                }
            },

            b'#' => {
                while self.peek() != b'\n' && !self.is_at_end() {
                    self.advance();
                }
            },

            b' ' => {
                match self.indent {
                    Indent::Neither => {
                        let mut spaces = 0;
                        while self.peek() == b' ' && !self.is_at_end() {
                            self.advance();
                            spaces += 1;
                        }
                        if self.is_at_end() {
                            return Err(Error::ParseError);
                        }
                        self.indent = Indent::Spaces(spaces);
                        self.add_token(TokenKind::Indent);
                    }
                    Indent::Spaces(s) => {
                        let mut spaces = 0;
                        while spaces <= s && self.peek() == b' ' && !self.is_at_end() {
                            self.advance();
                            spaces += 1;
                        }
                        if self.is_at_end() {
                            return Err(Error::ParseError);
                        }
                        if spaces != s {
                            return Err(Error::IndentationError);
                        }
                        self.add_token(TokenKind::Indent);
                    },
                    Indent::Tabs => return Err(Error::IndentationError)
                }
            },

            b'\t' => {
                match self.indent {
                    Indent::Neither => {
                        self.add_token(TokenKind::Indent);
                        self.indent = Indent::Tabs;
                    },
                    Indent::Tabs => {
                        self.add_token(TokenKind::Indent);
                    },
                    Indent::Spaces(_) => return Err(Error::IndentationError)
                }
            },

            b'\n' => self.add_token(TokenKind::Newline),

            _ => return Err(Error::ParseError)
        }
        Ok(())
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current]
    }

    fn add_token(&mut self, kind: TokenKind) {
        // assume valid UTF8
        let text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        self.tokens.push(Token::new(kind, text, self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected as u8 {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> u8 {
        if self.current >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }
}


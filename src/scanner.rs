
use token::{Token, TokenKind};
use error::{NcclError, ErrorKind};

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<NcclError>> {
        let mut err = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let e = self.scan_token();
            if e.is_err() {
                err.push(e.err().unwrap());
            }
        }

        self.tokens.push(Token::new(TokenKind::EOF, "".into(), self.line));

        if !err.is_empty() {
            Err(err)
        } else {
            Ok(self.tokens.clone())
        }
    }

    fn scan_token(&mut self) -> Result<(), NcclError> {
        let mut error = Ok(());
        match self.advance() {
            b':' => {
                self.add_token(TokenKind::Colon);
                while self.peek() == b' ' && !self.is_at_end() {
                    self.advance();
                }
                if self.peek() == b'\n' {
                    error = Err(NcclError::new(ErrorKind::ParseError, "Expected schema name, found newline", self.line));
                } else if self.is_at_end() {
                    error = Err(NcclError::new(ErrorKind::ParseError, "Expected schema name, found EOF", self.line));
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
                            error = Err(NcclError::new(ErrorKind::ParseError, "Expected value, found EOF", self.line));
                        }
                        self.indent = Indent::Spaces(spaces);
                        self.add_token(TokenKind::Indent);
                    }
                    Indent::Spaces(s) => {
                        let mut spaces = 0;
                        while spaces < s  && !self.is_at_end() {
                            if self.peek() != b' ' {
                                error = Err(NcclError::new(ErrorKind::IndentationError, &format!("Incorrect number of spaces: found {}, expected {}", spaces, s), self.line));
                            }
                            self.advance();
                            spaces += 1;
                        }
                        if self.is_at_end() {
                            error = Err(NcclError::new(ErrorKind::ParseError, "Expected value, found EOF", self.line));
                        }
                        self.add_token(TokenKind::Indent);
                    },
                    Indent::Tabs => { error = Err(NcclError::new(ErrorKind::IndentationError, "Expected tabs, found spaces", self.line)); }
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
                    Indent::Spaces(_) => { error = Err(NcclError::new(ErrorKind::IndentationError, "Expected spaces, found tabs", self.line)); }
                }
            },

            b'\n' => {
                self.add_token(TokenKind::Newline);
                self.line += 1;
            },

            b'\r' => {},

            b'"' => if let Err(e) = self.string() { error = Err(e); },

            _ => if let Err(e) = self.identifier() { error = Err(e); },
        }

        error
    }

    fn identifier(&mut self) -> Result<(), NcclError> {
        while self.peek() != b'\n' && self.peek() != b'\r' && self.peek() != b':' && !self.is_at_end() {
            self.advance();
        }

        let value = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        self.add_token_string(TokenKind::Value, value);

        Ok(())
    }

    fn string(&mut self) -> Result<(), NcclError> {
        let mut value = String::new();
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }

            if self.peek() == b'\\' {
                self.advance();
                match self.peek() {
                    b'n' => {
                        value.push('\n');
                    },
                    b'r' => {
                        value.push('\r');
                    }
                    b'\\' => {
                        value.push('\\');
                    },
                    b'"' => {
                        value.push('"');
                    },
                    _ => return Err(NcclError::new(ErrorKind::ParseError, &format!("Unknown format code: {}", self.peek()), self.line))
                }
            } else {
                value.push(self.source[self.current] as char);
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(NcclError::new(ErrorKind::ParseError, "Unterminated string", self.line));
        }

        self.advance();

        self.add_token_string(TokenKind::Value, value);

        Ok(())
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        self.tokens.push(Token::new(kind, text, self.line));
    }

    fn add_token_string(&mut self, kind: TokenKind, value: String) {
        self.tokens.push(Token::new(kind, value, self.line));
    }

    fn peek(&mut self) -> u8 {
        if self.current >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }
}


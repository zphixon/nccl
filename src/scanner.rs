use crate::error::{ErrorKind, NcclError};
use crate::token::{Span, Token, TokenKind};
use std::collections::VecDeque;

pub(crate) struct Scanner<'a> {
    source: &'a [u8],
    pub(crate) tokens: VecDeque<Token<'a>>,
    start: usize,
    current: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.as_bytes(),
            tokens: VecDeque::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn scan_all(mut self) -> Result<Vec<Token<'a>>, NcclError> {
        while self.next()?.kind != TokenKind::EOF {}
        Ok(self.tokens.drain(0..).collect())
    }

    pub(crate) fn next_token(&mut self) -> Result<Token<'a>, NcclError> {
        if self.tokens.is_empty() {
            self.next()?;
        }

        Ok(self.tokens.pop_front().unwrap())
    }

    pub(crate) fn peek_token(&mut self, idx: usize) -> Result<&Token<'a>, NcclError> {
        if self.tokens.is_empty() {
            self.next()?;
        }

        while self.tokens.len() <= idx {
            self.next()?;
        }

        Ok(&self.tokens[idx])
    }

    fn next(&mut self) -> Result<&Token<'a>, NcclError> {
        self.start = self.current;
        loop {
            match self.peek_char() {
                b'\0' => {
                    self.start = 0;
                    self.current = 0;
                    self.add_token(TokenKind::EOF)?;
                    return Ok(&self.tokens[self.tokens.len() - 1]);
                }

                b'\n' | b'\r' => {
                    self.advance_char();
                    self.start = self.current;
                }

                b'\t' => {
                    let mut tabs = 0;
                    while self.peek_char() == b'\t' {
                        self.advance_char();
                        tabs += 1;
                    }

                    if self.peek_char() == b'#'
                        || self.peek_char() == b'\n'
                        || self.peek_char() == b'\r'
                    {
                        self.until_newline();
                    } else {
                        self.add_token(TokenKind::Tabs(tabs))?;
                        break;
                    }
                }

                b' ' => {
                    let mut spaces = 0;
                    while self.peek_char() == b' ' {
                        self.advance_char();
                        spaces += 1;
                    }

                    if self.peek_char() == b'#'
                        || self.peek_char() == b'\n'
                        || self.peek_char() == b'\r'
                    {
                        self.until_newline();
                    } else {
                        self.add_token(TokenKind::Spaces(spaces))?;
                        break;
                    }
                }

                b'#' => {
                    self.until_newline();
                }

                _ => break,
            }
        }

        self.start = self.current;

        match self.peek_char() {
            quote @ (b'"' | b'\'') => self.string(quote)?,

            _ => {
                self.until_newline();
                self.add_token(TokenKind::Value)?;
            }
        }

        Ok(&self.tokens[self.tokens.len() - 1])
    }

    fn string(&mut self, quote: u8) -> Result<(), NcclError> {
        // go past the first quote
        self.advance_char();

        while self.peek_char() != quote && !self.is_at_end() {
            if self.peek_char() == b'\n' {
                self.line += 1;
            }

            if self.peek_char() == b'\\' {
                self.advance_char();
                match self.peek_char() {
                    b'n' | b'r' | b'\\' | b'"' => {}

                    b'\r' | b'\n' => {
                        self.advance_char();
                        while self.peek_char() == b' ' || self.peek_char() == b'\t' {
                            self.advance_char();
                        }
                        self.reverse_char();
                    }

                    _ => {
                        return Err(NcclError::new(
                            ErrorKind::Parse,
                            &format!("Unknown format code: {}", self.peek_char()),
                            self.line as u64,
                        ))
                    }
                }
            }

            self.advance_char();
        }

        // go past the last quote
        self.advance_char();

        self.add_token(TokenKind::Value)?;

        // go to the end of the line
        // prevent stuff like
        //     "hello" raw stuff out here
        // maybe it's fine? TODO

        while self.peek_char() == b' ' || self.peek_char() == b'\t' {
            self.advance_char();
        }

        if self.peek_char() == b'\n' || self.peek_char() == b'\r' {
            self.advance_char();
        } else if self.peek_char() == b'#' {
            self.until_newline();
        } else {
            return Err(NcclError::new(
                ErrorKind::Parse,
                "expected whitespace then comment or newline after string",
                self.line as u64,
            ));
        }

        Ok(())
    }

    fn until_newline(&mut self) {
        while self.peek_char() != b'\n' && self.peek_char() != b'\r' && !self.is_at_end() {
            self.advance_char();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance_char(&mut self) -> u8 {
        self.column += 1;
        self.current += 1;
        self.source[self.current - 1]
    }

    fn reverse_char(&mut self) -> u8 {
        self.current -= 1;
        self.source[self.current]
    }

    fn peek_char(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn add_token(&mut self, kind: TokenKind) -> Result<(), NcclError> {
        // TODO str::from_utf8 returns a different error than String::from_utf8?? why????????
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .map_err(|_err| NcclError::new(ErrorKind::Io, "invalid UTF-8", self.line as u64))?;

        self.tokens.push_back(Token {
            kind,
            lexeme,
            span: Span {
                line: self.line,
                column: self.column,
            },
        });

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_all(source: &str) -> Vec<(TokenKind, &str)> {
        Scanner::new(source)
            .scan_all()
            .unwrap()
            .into_iter()
            .map(|token| (token.kind, token.lexeme))
            .collect::<Vec<_>>()
    }

    #[test]
    fn empty() {
        use super::TokenKind::*;
        let file = std::fs::read_to_string("examples/empty.nccl").unwrap();
        let tokens = get_all(&file);
        assert_eq!(tokens, vec![(EOF, "")]);
    }

    #[test]
    fn oh_lord() {
        use super::TokenKind::*;
        let file = std::fs::read_to_string("examples/funky-indent.nccl").unwrap();
        let tokens = get_all(&file);
        assert_eq!(tokens, vec![(Value, "a"), (Value, "b"), (EOF, "")]);
    }

    #[test]
    fn tabbies() {
        use super::TokenKind::*;

        let file = std::fs::read_to_string("examples/good-tabs.nccl").unwrap();
        let tokens = get_all(&file);

        #[rustfmt::skip]
        assert_eq!(
            tokens,
            vec![
                (Value, "jackson"),
                (Tabs(1), "\t"), (Value, "easy"),
                (Tabs(2), "\t\t"), (Value, "abc"),
                (Tabs(2), "\t\t"), (Value, "123"),
                (Tabs(1), "\t"), (Value, "hopefully"),
                (Tabs(2), "\t\t"), (Value, "tabs work"),
                (EOF, ""),
            ]
        );
    }

    #[test]
    fn new_scan() {
        use super::TokenKind::*;

        let file = std::fs::read_to_string("examples/all-of-em.nccl").unwrap();
        let tokens = get_all(&file);

        #[rustfmt::skip]
        assert_eq!(
            tokens,
            vec![
                (Value, "a"),
                (Spaces(4), "    "), (Value, "b"),
                (Spaces(4), "    "), (Value, "c"),
                (Value, "d"),
                (Spaces(2), "  "), (Value, "e"),
                (Spaces(2), "  "), (Value, "f"),
                (Value, "h"),
                (Tabs(1), "\t"), (Value, "i # j"),
                (Tabs(1), "\t"), (Value, "\"k\""),
                (Tabs(1), "\t"), (Value, "'m'"),
                (Value, "o"),
                (EOF, ""),
            ]
        );
    }
}

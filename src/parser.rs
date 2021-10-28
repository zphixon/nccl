use crate::error::{ErrorKind, NcclError};
use crate::pair::Pair;
use crate::scanner::Scanner2;
use crate::token::{Token, Token2, TokenKind};
use crate::value::{parse_into_value, Value};
use crate::{parse_config, Config};

pub(crate) fn parse<'a>(scanner: &mut Scanner2<'a>) -> Result<Config<'a, 'a>, NcclError> {
    parse_with(scanner, &Config::new("__top_level__"))
}

pub(crate) fn parse_with<'orig, 'new>(
    scanner: &mut Scanner2<'new>,
    original: &Config<'orig, 'new>,
) -> Result<Config<'new, 'new>, NcclError> {
    let mut config = original.clone();

    while scanner.peek_token(0)?.kind != TokenKind::EOF {
        parse_kv(scanner, 0, &mut config)?;
    }

    Ok(config)
}

fn parse_kv<'a>(
    scanner: &mut Scanner2<'a>,
    indent: u8,
    parent: &mut Config<'a, 'a>,
) -> Result<(), NcclError> {
    // TODO spaces
    let mut node = Config::new(consume(scanner, TokenKind::Value)?.lexeme);
    while scanner.peek_token(0)?.kind == TokenKind::Tab(indent + 1) {
        consume(scanner, TokenKind::Tab(indent + 1)).unwrap();
        parse_kv(scanner, indent + 1, &mut node)?;
    }
    parent.add_child(node);
    Ok(())
}

fn consume<'a>(scanner: &mut Scanner2<'a>, kind: TokenKind) -> Result<Token2<'a>, NcclError> {
    let tok = scanner.next_token()?;
    if tok.kind == kind {
        Ok(tok)
    } else {
        Err(NcclError::new(
            ErrorKind::Parse,
            &format!("expected {:?}, got {:?}", kind, tok),
            scanner.line as u64,
        ))
    }
}

#[derive(Debug)]
pub struct Parser {
    current: usize,
    path: Vec<Value>,
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
            tokens,
            pair: Pair::new("__top_level__"),
            line: 1,
        }
    }

    pub fn new_with(tokens: Vec<Token>, pair: Pair) -> Self {
        Parser {
            current: 0,
            path: vec![],
            indent: 0,
            tokens,
            pair,
            line: 1,
        }
    }

    pub fn parse(mut self) -> Result<Pair, Vec<NcclError>> {
        let mut errors = vec![];
        let mut prev_indent = 0;
        let mut i = 0;

        while i < self.tokens.len() {
            match self.tokens[i].kind {
                TokenKind::Tab(_) | TokenKind::Space(_) => unimplemented!(),

                TokenKind::Value => {
                    // add to path respective of self.index
                    if self.indent <= self.path.len() {
                        let mut new = self.path[0..self.indent].to_owned();
                        new.push(parse_into_value(self.tokens[i].lexeme.clone()));
                        self.path = new;
                    } else {
                        self.path
                            .push(parse_into_value(self.tokens[i].lexeme.clone()));
                    }

                    self.pair.add_slice(&self.path);

                    if i + 2 < self.tokens.len() && self.tokens[i + 2].kind == TokenKind::Value {
                        self.path.clear();
                        self.indent = 0;
                    }
                }

                TokenKind::Indent => {
                    // set new self.index
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
                            errors.push(NcclError::new(
                                ErrorKind::Indentation,
                                "Incorrect level of indentation found",
                                self.line,
                            ));
                            self.indent = prev_indent;
                        }
                    } else if indent - prev_indent == 1 || indent - prev_indent == 0 {
                        self.indent = indent;
                    } else {
                        errors.push(NcclError::new(
                            ErrorKind::Indentation,
                            "Incorrect level of indentation found",
                            self.line,
                        ));
                        self.indent = prev_indent;
                    }
                }

                TokenKind::Newline => {
                    // reset self.index
                    prev_indent = self.indent;
                    self.indent = 0;
                    self.line += 1;
                }

                TokenKind::EOF => break,
            }
            i += 1;
        }

        if errors.is_empty() {
            Ok(self.pair)
        } else {
            Err(errors)
        }
    }
}

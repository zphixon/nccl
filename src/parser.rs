use crate::error::{ErrorKind, NcclError};
use crate::pair::Pair;
use crate::token::{Token, Token2, TokenKind};
use crate::value::{parse_into_value, Value};
use crate::{parse_config, Config};

/* __top_level__
 *      hello
 *          world
 *              panama
 *          friends
 *              doggos
 *      sandwich
 *          meat
 *              bologne
 *              ham
 *          cheese
 *              provolone
 *              cheddar
 */

pub(crate) fn parse<'a>(tokens: &[Token2<'a>]) -> Result<Config<'a, 'a>, NcclError> {
    let mut config = Config::new("__top_level__");
    do_parse(tokens, &mut config);
    Ok(config)
}

pub(crate) fn parse_with<'orig, 'new>(
    tokens: &[Token2<'new>],
    original: &Config<'orig, 'new>,
) -> Result<Config<'new, 'new>, NcclError> {
    let mut config = original.clone();
    do_parse(tokens, &mut config)?;
    Ok(config)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Indent {
    TopLevel,
    Tabs { level: usize },
    Spaces { number: u8, level: usize },
}

fn do_parse<'a>(tokens: &[Token2<'a>], config: &mut Config<'a, 'a>) -> Result<(), NcclError> {
    do_parse_indent(tokens, config, Indent::TopLevel)
}

fn do_parse_indent<'a>(
    tokens: &[Token2<'a>],
    config: &mut Config<'a, 'a>,
    indent: Indent,
) -> Result<(), NcclError> {
    for (i, token) in tokens.iter().enumerate() {
        println!("{} {:?}", i, token);
        match token.kind {
            TokenKind::Value => config.add_value(token.lexeme),

            TokenKind::Tab(num_tabs) => {
                do_parse_indent(
                    &tokens[i + 1..],
                    config,
                    match indent {
                        Indent::TopLevel => Indent::Tabs { level: 0 },

                        Indent::Tabs { level } => {
                            if num_tabs as usize == level + 1 {
                                Indent::Tabs { level: level + 1 }
                            } else {
                                return Err(NcclError::new(
                                    ErrorKind::Indentation,
                                    &format!("expected {} tabs, got {}", level + 1, num_tabs),
                                    token.span.line as u64,
                                ));
                            }
                        }

                        Indent::Spaces { .. } => {
                            return Err(NcclError::new(
                                ErrorKind::Indentation,
                                "expected tabs, found spaces",
                                token.span.line as u64,
                            ));
                        }
                    },
                )?;
            }

            TokenKind::Space(number) => {
                todo!();
                //if indent == Indent::TopLevel {
                //    indent = Indent::Spaces { number, level: 0 };
                //}
                //
                //if matches!(indent, Indent::Tabs { .. }) {
                //    return Err(NcclError::new(
                //        ErrorKind::Indentation,
                //        "expected spaces, found tabs",
                //        token.span.line as u64,
                //    ));
                //}
            }

            TokenKind::Newline => {}

            _ => panic!(),
            //TokenKind::Tab
        }
    }

    Ok(())
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

use crate::error::{ErrorKind, NcclError};
use crate::pair::Pair;
use crate::scanner::Scanner2;
use crate::token::{Token, Token2, TokenKind};
use crate::value::{parse_into_value, Value};
use crate::Config;

#[derive(Clone, Copy)]
enum Indent {
    TopLevel,
    Tabs { level: u8 },
    Spaces { width: u8, level: u8 },
}

impl Indent {
    fn level_tabs(&self) -> u8 {
        match self {
            Indent::TopLevel => 0,
            &Indent::Tabs { level } => level,
            Indent::Spaces { .. } => unreachable!(),
        }
    }

    fn level_spaces(&self) -> u8 {
        match self {
            Indent::TopLevel => 0,
            Indent::Tabs { .. } => unreachable!(),
            Indent::Spaces { width, level } => width * level,
        }
    }

    fn width(&self) -> Option<u8> {
        match self {
            Indent::TopLevel => None,
            Indent::Tabs { .. } => unreachable!(),
            &Indent::Spaces { width, .. } => Some(width),
        }
    }

    fn increase_tabs(&self) -> Indent {
        match self {
            Indent::TopLevel => Indent::Tabs { level: 1 },
            Indent::Tabs { level } => Indent::Tabs { level: level + 1 },
            &Indent::Spaces { width, level } => Indent::Spaces {
                width,
                level: level + 1,
            },
        }
    }

    fn increase_spaces(&self, width: u8) -> Indent {
        match self {
            Indent::TopLevel => Indent::Spaces { width, level: 1 },
            Indent::Tabs { level } => Indent::Tabs { level: level + 1 },
            &Indent::Spaces { width, level } => Indent::Spaces {
                width,
                level: level + 1,
            },
        }
    }
}

pub(crate) fn parse<'a>(scanner: &mut Scanner2<'a>) -> Result<Config<'a, 'a>, NcclError> {
    parse_with(scanner, &Config::new("__top_level__"))
}

pub(crate) fn parse_with<'orig, 'new>(
    scanner: &mut Scanner2<'new>,
    original: &Config<'orig, 'new>,
) -> Result<Config<'new, 'new>, NcclError> {
    let mut config = original.clone();

    while scanner.peek_token(0)?.kind != TokenKind::EOF {
        parse_kv(scanner, Indent::TopLevel, &mut config)?;
    }

    Ok(config)
}

fn parse_kv<'a>(
    scanner: &mut Scanner2<'a>,
    indent: Indent,
    parent: &mut Config<'a, 'a>,
) -> Result<(), NcclError> {
    let value = consume(scanner, TokenKind::Value)?.lexeme;
    let mut node = {
        if parent.has_value(value) {
            parent[value].clone()
        } else {
            Config::new(value)
        }
    };

    match scanner.peek_token(0)?.kind {
        TokenKind::Tabs(tabs) => {
            let next_indent = indent.increase_tabs();
            if tabs == next_indent.level_tabs() {
                while scanner.peek_token(0)?.kind == TokenKind::Tabs(next_indent.level_tabs()) {
                    consume(scanner, TokenKind::Tabs(next_indent.level_tabs())).unwrap();
                    parse_kv(scanner, next_indent, &mut node)?;
                }
            }
        }

        TokenKind::Spaces(spaces) if matches!(indent, Indent::Spaces { .. } | Indent::TopLevel) => {
            let next_indent = indent.increase_spaces(indent.width().unwrap_or(spaces));
            if spaces == next_indent.level_spaces() {
                while scanner.peek_token(0)?.kind == TokenKind::Spaces(next_indent.level_spaces()) {
                    consume(scanner, TokenKind::Spaces(next_indent.level_spaces())).unwrap();
                    parse_kv(scanner, next_indent, &mut node)?;
                }
            }
        }

        _ => {}
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! map {
        ($($key:expr => $item:expr),*) => {
            {
                #[allow(unused_mut)]
                let mut set = crate::pair::make_map();
                $(
                    set.insert($key, $item);
                )*
                set
            }
        }
    }

    #[test]
    fn tab_config() {
        let source = std::fs::read_to_string("examples/good-tabs.nccl").unwrap();
        let mut scanner = Scanner2::new(&source);
        let config = parse(&mut scanner).unwrap();
        assert_eq!(
            config,
            Config {
                key: "__top_level__",
                value: map![
                    "jackson" => Config {
                        key: "jackson",
                        value: map![
                            "easy" => Config {
                                key: "easy",
                                value: map![
                                    "abc" => Config {
                                        key: "abc",
                                        value: map![]
                                    },
                                    "123" => Config {
                                        key: "123",
                                        value: map![]
                                    }
                                ]
                            },
                            "hopefully" => Config {
                                key: "hopefully",
                                value: map![
                                    "tabs work" => Config {
                                        key: "tabs work",
                                        value: map![]
                                    }
                                ]
                            }
                        ],
                    }
                ]
            }
        );
    }

    #[test]
    fn pconfig() {
        let source = std::fs::read_to_string("examples/config.nccl").unwrap();
        let mut scanner = Scanner2::new(&source);
        let config = parse(&mut scanner).unwrap();
        assert_eq!(
            config,
            Config {
                key: "__top_level__",
                value: map![
                    "server" => Config {
                        key: "server",
                        value: map![
                            "domain" => Config {
                                key: "domain",
                                value: map![
                                    "example.com" => Config {
                                        key: "example.com",
                                        value: map![]
                                    },
                                    "www.example.com" => Config {
                                        key: "www.example.com",
                                        value: map![]
                                    }
                                ]
                            },
                            "port" => Config {
                                key: "port",
                                value: map![
                                    "80" => Config {
                                        key: "80",
                                        value: map![]
                                    },
                                    "443" => Config {
                                        key: "443",
                                        value: map![]
                                    }
                                ]
                            },
                            "root" => Config {
                                key: "root",
                                value: map![
                                    "/var/www/html" => Config {
                                        key: "/var/www/html",
                                        value: map![]
                                    }
                                ]
                            }
                        ],
                    }
                ]
            }
        );
    }

    #[test]
    fn woke() {
        let dir = std::fs::read_dir("examples").unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                println!("check good: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let mut scanner = Scanner2::new(&source);
                parse(&mut scanner).unwrap();
            }
        }
    }

    #[test]
    fn broke() {
        let dir = std::fs::read_dir("examples/bad").unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            println!("check is bad: {}", entry.path().display());
            let source = std::fs::read_to_string(entry.path()).unwrap();
            let mut scanner = Scanner2::new(&source);
            parse(&mut scanner).unwrap_err();
        }
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
                TokenKind::Tabs(_) | TokenKind::Spaces(_) => unimplemented!(),

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

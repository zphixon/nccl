//! Module containing the parser

use crate::scanner::Scanner;
use crate::scanner::{Token, TokenKind};
use crate::Config;
use crate::NcclError;

/// The key of the top-level node.
pub const TOP_LEVEL_KEY: &str = "__top_level__";

#[derive(Clone, Copy)]
enum Indent {
    TopLevel,
    Tabs { level: usize },
    Spaces { width: usize, level: usize },
}

impl Indent {
    fn level_tabs(&self) -> usize {
        match self {
            Indent::TopLevel => 0,
            &Indent::Tabs { level } => level,
            Indent::Spaces { .. } => unreachable!(),
        }
    }

    fn level_spaces(&self) -> usize {
        match self {
            Indent::TopLevel => 0,
            Indent::Tabs { .. } => unreachable!(),
            Indent::Spaces { width, level } => width * level,
        }
    }

    fn width(&self) -> Option<usize> {
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

    fn increase_spaces(&self, width: usize) -> Indent {
        match self {
            Indent::TopLevel => Indent::Spaces { width, level: 1 },
            Indent::Tabs { level } => Indent::Tabs { level: level + 1 },
            &Indent::Spaces { width, level } => Indent::Spaces {
                width,
                level: level + 1,
            },
        }
    }

    fn is_tabs_or_top_level(&self) -> bool {
        matches!(self, Indent::Tabs { .. }) || matches!(self, Indent::TopLevel)
    }

    fn is_spaces_or_top_level(&self) -> bool {
        matches!(self, Indent::Spaces { .. }) || matches!(self, Indent::TopLevel)
    }
}

pub(crate) fn parse<'a>(scanner: &mut Scanner<'a>) -> Result<Config<'a>, NcclError> {
    parse_with(scanner, &Config::new(TOP_LEVEL_KEY, None))
}

pub(crate) fn parse_with<'a>(
    scanner: &mut Scanner<'a>,
    original: &Config<'a>,
) -> Result<Config<'a>, NcclError> {
    let mut config = original.clone();

    while scanner.peek_token(0)?.kind != TokenKind::Eof {
        parse_kv(scanner, Indent::TopLevel, &mut config)?;
    }

    Ok(config)
}

fn parse_kv<'a>(
    scanner: &mut Scanner<'a>,
    indent: Indent,
    parent: &mut Config<'a>,
) -> Result<(), NcclError> {
    let value = consume_value(scanner)?;
    let mut node = {
        if parent.has_value(value.lexeme) {
            parent[value.lexeme].clone()
        } else if let TokenKind::QuotedValue(kind) = value.kind {
            Config::new_with_span(value.lexeme, value.span, Some(kind))
        } else {
            Config::new_with_span(value.lexeme, value.span, None)
        }
    };

    match scanner.peek_token(0)?.kind {
        TokenKind::Tabs(tabs) if indent.is_tabs_or_top_level() => {
            let next_indent = indent.increase_tabs();
            if tabs == next_indent.level_tabs() {
                while scanner.peek_token(0)?.kind == TokenKind::Tabs(next_indent.level_tabs()) {
                    consume(scanner, TokenKind::Tabs(next_indent.level_tabs())).unwrap();
                    parse_kv(scanner, next_indent, &mut node)?;
                }
            }
        }

        //TokenKind::Spaces(spaces) if matches!(indent, Indent::Spaces { .. } | Indent::TopLevel) => {
        TokenKind::Spaces(spaces) if indent.is_spaces_or_top_level() => {
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

fn consume_value<'a>(scanner: &mut Scanner<'a>) -> Result<Token<'a>, NcclError> {
    let tok = scanner.next_token()?;
    match tok.kind {
        TokenKind::Value | TokenKind::QuotedValue(_) => Ok(tok),
        _ => Err(NcclError::UnexpectedToken {
            span: tok.span,
            expected: TokenKind::Value,
            got: tok.kind,
        }),
    }
}

fn consume<'a>(scanner: &mut Scanner<'a>, kind: TokenKind) -> Result<Token<'a>, NcclError> {
    let tok = scanner.next_token()?;
    if tok.kind == kind {
        Ok(tok)
    } else {
        Err(NcclError::UnexpectedToken {
            span: tok.span,
            expected: kind,
            got: tok.kind,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Span;

    macro_rules! map {
        ($($key:expr => $item:expr),*) => {
            {
                #[allow(unused_mut)]
                let mut set = crate::config::make_map();
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
        let mut scanner = Scanner::new(&source);
        let config = parse(&mut scanner).unwrap();
        assert_eq!(
            config,
            Config {
                quotes: None,
                key: TOP_LEVEL_KEY,
                span: Span::default(),
                value: map![
                    "jackson" => Config {
                        quotes: None,
                        key: "jackson",
                        span: Span::default(),
                        value: map![
                            "easy" => Config {
                                quotes: None,
                                key: "easy",
                                span: Span::default(),
                                value: map![
                                    "abc" => Config {
                                        quotes: None,
                                        key: "abc",
                                        span: Span::default(),
                                        value: map![]
                                    },
                                    "123" => Config {
                                        quotes: None,
                                        key: "123",
                                        span: Span::default(),
                                        value: map![]
                                    }
                                ]
                            },
                            "hopefully" => Config {
                                quotes: None,
                                key: "hopefully",
                                span: Span::default(),
                                value: map![
                                    "tabs work" => Config {
                                        quotes: None,
                                        key: "tabs work",
                                        span: Span::default(),
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
        let mut scanner = Scanner::new(&source);
        let config = parse(&mut scanner).unwrap();
        assert_eq!(
            config,
            Config {
                quotes: None,
                key: TOP_LEVEL_KEY,
                span: Span::default(),
                value: map![
                    "server" => Config {
                        quotes: None,
                        key: "server",
                        span: Span::default(),
                        value: map![
                            "domain" => Config {
                                quotes: None,
                                key: "domain",
                                span: Span::default(),
                                value: map![
                                    "example.com" => Config {
                                        quotes: None,
                                        key: "example.com",
                                        span: Span::default(),
                                        value: map![]
                                    },
                                    "www.example.com" => Config {
                                        quotes: None,
                                        key: "www.example.com",
                                        span: Span::default(),
                                        value: map![]
                                    }
                                ]
                            },
                            "port" => Config {
                                quotes: None,
                                key: "port",
                                span: Span::default(),
                                value: map![
                                    "80" => Config {
                                        quotes: None,
                                        key: "80",
                                        span: Span::default(),
                                        value: map![]
                                    },
                                    "443" => Config {
                                        quotes: None,
                                        key: "443",
                                        span: Span::default(),
                                        value: map![]
                                    }
                                ]
                            },
                            "root" => Config {
                                quotes: None,
                                key: "root",
                                span: Span::default(),
                                value: map![
                                    "/var/www/html" => Config {
                                        quotes: None,
                                        key: "/var/www/html",
                                        span: Span::default(),
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
            if entry.metadata().unwrap().is_file()
                && entry.path().display().to_string().ends_with("nccl")
            {
                println!("check good: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let mut scanner = Scanner::new(&source);
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
            let mut scanner = Scanner::new(&source);
            parse(&mut scanner).unwrap_err();
        }
    }
}

//! A simple configuration language.
//!
//! Nccl is an easy way to add minimal configuration to your crate without
//! having to deal with complicated interfaces, obnoxious syntax, or outdated
//! languages. Nccl makes it easy for a user to pick up your configuration.
//! It's as easy as five cents.
//!
//! Nccl was motivated by the fact that other configuration languages are too
//! complicated for both end-users and developers. Everyone loves types, but
//! in a configuration language, there's too much room for interpretation
//! There are no types here.
//!
//! This is a nice approach for smaller, non-semantic configurations; you
//! shouldn't be accidentally implementing a DSL with nccl, and if you are, it
//! should feel painful.
//!
//! For interacting with a parsed configuration, see [`config::Config`].
//!
//! ## Syntax
//!
//! The single most important feature:
//!
//! ```text
//! key
//!     value
//! ```
//!
//! ```rust
//! let source = r#"
//! key
//!     value
//! "#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert_eq!(config["key"].value(), Some("value"));
//! ```
//!
//! Results in `config["key"].value() == Some("value")`. Note that there is no
//! semantic difference between a key and a value, so `config.value() == Some("key")`.
//! For most of this document and the library source code, the words key, value, and
//! node are used almost interchangeably.
//!
//! There are no types:
//!
//! ```rust
//! let source = r#"
//! threads
//!     16
//! is this a problem?
//!     no 🇳🇴
//! end of the world
//!     2012-12-21"#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert_eq!(config["is this a problem?"].value(), Some("no 🇳🇴"));
//! ```
//!
//! Interpret the data however you want.
//!
//! Indentation is significant. Each top-level key-value pair must use the same
//! type of indentation for each sub-value. You may mix indentation per-file.
//!
//! ```rust
//! let source = r#"
//! ## tab
//! a
//! 	1
//!
//! ## single space
//! b
//!  2
//! "#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert_eq!(config["a"].value(), Some("1"));
//! assert_eq!(config["b"].value(), Some("2"));
//! ```
//!
//! Comments exist on their own line or after a quoted value, otherwise they
//! become part of the value.
//!
//! ```rust
//! let source = r#"
//! hello # this is part of the key!
//!     ## this is not
//!     world
//!     "y'all" # this isn't either
//! "#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert!(config.has_value("hello # this is part of the key!"));
//! assert!(!config["hello # this is part of the key!"].has_value("# this is not"));
//! assert!(config["hello # this is part of the key!"].has_value("y'all"));
//! ```
//!
//! Duplicate keys have their values merged.
//!
//! ```rust
//! let source = r#"
//! oh christmas tree
//!     o tannenbaum
//!
//! oh christmas tree
//!     o tannenbaum
//!     five golden rings
//!     wait wrong song
//! "#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert_eq!(
//!     vec!["o tannenbaum", "five golden rings", "wait wrong song"],
//!     config["oh christmas tree"].values().collect::<Vec<_>>()
//! );
//! ```
//!
//! Results in one key "oh christmas tree" with three values. This property
//! enables [`parse_config_with`] to merge two configurations together. Say if
//! you wanted to enable an end user to be able to override some default values,
//! first you would parse the user's configuration, and then parse the default
//! on top of that. [`config::Config::value`] always returns the first value,
//! which would be the user's value.
//!
//! Values can have quotes if you want escape codes or multiple lines.
//! Supported escape sequences are newlines, carriage returns, both quotes, and
//! line breaks.
//!
//! ```rust
//! let source = r#"
//! ## both single and double quotes work
//! i can
//!     ## with parse_quoted(), expands to the rust string "show\nyou"
//!     'show\nyou'
//!     ## backslash followed by newline replaces all following whitespace
//!     ## except newlines with one space character. expands to "the world"
//!     "the \
//!     world"
//!
//! ## results in a single value for jingle = jangle
//! jingle
//!     jangle
//!     "jangle"
//!     'jangle'
//! "#;
//! let config = nccl::parse_config(&source).unwrap();
//! assert_eq!(1, config["jingle"].values().count());
//! assert_eq!(
//!     Ok(vec![String::from("show\nyou"), String::from("the world")]),
//!     config["i can"]
//!         .children()
//!         .map(|value| value.parse_quoted())
//!         .collect::<Result<Vec<_>, _>>()
//! );
//! ```

#![allow(clippy::tabs_in_doc_comments)]

pub mod config;
pub mod parser;
pub mod scanner;

pub use config::Config;

use scanner::{Span, TokenKind};

use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// Parse a nccl configuration
///
/// e.g.
/// ```
/// # use nccl::*;
/// // config.nccl:
/// // server
/// //     domain
/// //         example.com
/// //         www.example.com
/// //     port
/// //         80
/// //         443
/// //     root
/// //         /var/www/html
///
/// // read the config file
/// let content = std::fs::read_to_string("examples/config.nccl").unwrap();
///
/// // parse it
/// let config = parse_config(&content).unwrap();
///
/// // look ma, no types!
/// assert_eq!(config["server"]["root"].value(), Some("/var/www/html"));
/// ```
pub fn parse_config(content: &str) -> Result<Config, NcclError> {
    let mut scanner = scanner::Scanner::new(content);
    parser::parse(&mut scanner)
}

/// Parse a new nccl configuration on top of another
///
/// e.g.
/// ```
/// # use nccl::*;
/// // user.nccl:
/// // beans
/// //    four
///
/// // default.nccl:
/// // frog
/// //     yes
/// // beans
/// //     none
///
/// // result:
/// // frog
/// //     yes
/// // beans
/// //     four
/// //     none
///
/// // first get the user config
/// let user = std::fs::read_to_string("examples/user.nccl").unwrap();
/// let user_config = parse_config(&user).unwrap();
///
/// // then merge the default config on top of the user config
/// let default = std::fs::read_to_string("examples/default.nccl").unwrap();
/// let combined_config = parse_config_with(&user_config, &default).unwrap();
///
/// // with value(), the first key inserted is returned. since we read the user
/// // config first, the user-supplied value is first, overriding the default.
/// assert_eq!(combined_config["beans"].value(), Some("four"));
/// // "beans" now has two values
/// assert_eq!(combined_config["beans"].values().count(), 2);
///
/// // and the unmodified key remains
/// assert_eq!(combined_config["frog"].value(), Some("yes"));
/// ```
pub fn parse_config_with<'a>(
    config: &Config<'a>,
    content: &'a str,
) -> Result<Config<'a>, NcclError> {
    let mut scanner = scanner::Scanner::new(content);
    parser::parse_with(&mut scanner, config)
}

#[derive(Debug, PartialEq)]
/// Errors that may occur while parsing
pub enum NcclError {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The location of the token.
        span: Span,
        /// The kind of token we expected.
        expected: TokenKind,
        /// The kind of token we got.
        got: TokenKind,
    },
    /// The string was not terminated before the end of the file.
    UnterminatedString {
        /// The line the string starts on.
        start: usize,
    },
    /// There were non-comment characters after a quoted string.
    TrailingCharacters {
        /// The line the string ends on.
        line: usize,
    },
    /// The escape code in the file was unknown.
    ScanUnknownEscape {
        /// The line of the code.
        line: usize,
        /// The column of the code.
        column: usize,
        /// The code itself.
        escape: char,
    },
    /// The escape literal in the key was unknown. See [`crate::config::Config::parse_quoted`].
    ParseUnknownEscape {
        /// The escape code.
        escape: char,
    },
    /// A utf-8 string could not be constructed.
    Utf8 {
        /// The error.
        err: Utf8Error,
    },
}

impl std::fmt::Display for NcclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NcclError::UnexpectedToken {
                span,
                expected,
                got,
            } => write!(
                f,
                "expected {:?}, got {:?} at {}:{}",
                expected, got, span.line, span.column,
            ),
            NcclError::UnterminatedString { start } => {
                write!(f, "unterminated string starting on line {}", start)
            }
            NcclError::TrailingCharacters { line } => {
                write!(f, "characters after string on line {}", line)
            }
            NcclError::ScanUnknownEscape {
                escape,
                line,
                column,
            } => write!(f, "unknown escape {:?} at {}:{}", escape, line, column),
            NcclError::ParseUnknownEscape { escape } => write!(f, "unknown escape {:?}", escape),
            NcclError::Utf8 { err } => write!(f, "{}", err),
        }
    }
}

impl From<Utf8Error> for NcclError {
    fn from(err: Utf8Error) -> Self {
        NcclError::Utf8 { err }
    }
}

impl From<FromUtf8Error> for NcclError {
    fn from(err: FromUtf8Error) -> Self {
        NcclError::Utf8 {
            err: err.utf8_error(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn index() {
        let content = read_to_string("examples/config.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(config["server"]["root"].value(), Some("/var/www/html"));
    }

    #[test]
    fn values() {
        let content = read_to_string("examples/config.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(
            vec![80, 443],
            config["server"]["port"]
                .values()
                .map(|port| port.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
                .unwrap()
        );
    }

    #[test]
    fn value() {
        let content = read_to_string("examples/long.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(config["bool too"].value().unwrap(), "false");
    }

    #[test]
    fn duplicates() {
        let content = read_to_string("examples/duplicates.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(
            config["something"].values().collect::<Vec<_>>(),
            vec!["with", "duplicates"]
        );
    }

    #[test]
    fn duplicates2() {
        let content1 = read_to_string("examples/duplicates.nccl").unwrap();
        let config1 = parse_config(&content1).unwrap();

        let content2 = read_to_string("examples/duplicates2.nccl").unwrap();
        let config2 = parse_config_with(&config1, &content2).unwrap();

        assert_eq!(2, config2["something"].values().collect::<Vec<_>>().len());
    }

    #[test]
    fn duplicates3() {
        let content1 = read_to_string("examples/dup3.nccl").unwrap();
        let config1 = parse_config(&content1).unwrap();

        assert_eq!(
            vec!["oh christmas tree", "o tannenbaum", "five golden rings"],
            config1["oh christmas tree"].values().collect::<Vec<_>>()
        );
    }

    #[test]
    fn inherit() {
        let sc = read_to_string("examples/inherit.nccl").unwrap();
        let uc = read_to_string("examples/inherit2.nccl").unwrap();

        let schema = parse_config(&sc).unwrap();
        let user = parse_config_with(&schema, &uc).unwrap();

        assert_eq!(3, user["hello"]["world"].values().collect::<Vec<_>>().len());
        assert_eq!(
            3,
            user["sandwich"]["meat"].values().collect::<Vec<_>>().len()
        );
    }

    #[test]
    fn comments() {
        let config = r#"x
# comment
    something
    # comment again
        bingo

does this work?
    who knows
# I sure don't
    is this a child?
"#;
        let config = parse_config(config).unwrap();

        assert_eq!(config["x"]["something"].value().unwrap(), "bingo");
        assert!(config["does this work?"].has_value("who knows"));
        assert!(config["does this work?"].has_value("is this a child?"));
    }

    #[test]
    fn all_of_em() {
        let source = read_to_string("examples/all-of-em.nccl").unwrap();
        let mut scanner = scanner::Scanner::new(&source);
        let config = parser::parse(&mut scanner).unwrap();
        assert_eq!(
            Ok(vec![
                String::from("i # j"),
                String::from("k"),
                String::from("m")
            ]),
            config["h"]
                .children()
                .map(|config| config.parse_quoted())
                .collect::<Result<Vec<_>, _>>()
        );
        assert_eq!(Some(scanner::QuoteKind::Double), config["h"]["k"].quotes);
        assert_eq!(Some(scanner::QuoteKind::Single), config["h"]["m"].quotes);
    }

    #[test]
    fn escapes() {
        let config = read_to_string("examples/escapes.nccl").unwrap();
        let config = parse_config(&config).unwrap();
        assert_eq!(
            config["hello"].child().unwrap().parse_quoted().unwrap(),
            "people of the earth\nhow's it doing?\""
        );
        assert_eq!(
            config["hello"].child().unwrap().quotes,
            Some(scanner::QuoteKind::Double)
        );
    }

    #[test]
    fn quote() {
        let config = read_to_string("examples/quote.nccl").unwrap();
        let config = parse_config(&config).unwrap();
        assert_eq!(config["howdy"].values().collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn fuzz() {
        let dir = std::fs::read_dir("examples/fuzz/scan").unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            if entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("err")
            {
                println!("check scan bad: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let result = scanner::Scanner::new(&source).scan_all();
                println!("    {:?}", result);
                result.unwrap_err();
            } else {
                println!("check scan good: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let result = scanner::Scanner::new(&source).scan_all();
                result.unwrap();
            }
        }

        let dir = std::fs::read_dir("examples/fuzz/parse").unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            if entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("err")
            {
                println!("check parse bad: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let result = parse_config(&source);
                println!("    {:#?}", result);
                result.unwrap_err();
            } else {
                println!("check parse good: {}", entry.path().display());
                let source = std::fs::read_to_string(entry.path()).unwrap();
                let result = parse_config(&source);
                println!("    {:#?}", result);
                result.unwrap();
            }
        }
    }
}

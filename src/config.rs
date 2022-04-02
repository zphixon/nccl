//! Contains the configuration struct

use crate::parser::TOP_LEVEL_KEY;
use crate::scanner::QuoteKind;
use crate::NcclError;

use std::hash::{Hash, Hasher};
use std::ops::Index;

#[cfg(not(fuzzing))]
use indexmap::IndexMap;

/// Type alias for an [`IndexMap`], a hash map where insertion order is preserved.
#[cfg(not(fuzzing))]
pub type HashMap<K, V> = IndexMap<K, V, fnv::FnvBuildHasher>;

#[cfg(not(fuzzing))]
pub(crate) fn make_map<K, V>() -> HashMap<K, V> {
    HashMap::with_hasher(fnv::FnvBuildHasher::default())
}

#[cfg(fuzzing)]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(fuzzing)]
pub(crate) fn make_map<K, V>() -> HashMap<K, V> {
    HashMap::default()
}

/// A nccl configuration
///
/// Indexable with `&str`.
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
/// let content = std::fs::read_to_string("examples/config.nccl").unwrap();
/// let config = parse_config(&content).unwrap();
///
/// // get the value of a single node
/// assert_eq!(Some("/var/www/html"), config["server"]["root"].value());
///
/// // value always returns the value of the first child node
/// assert_eq!(Some("example.com"), config["server"]["domain"].value());
///
/// // get multiple values
/// assert_eq!(
///     vec!["example.com", "www.example.com"],
///     config["server"]["domain"].values().collect::<Vec<_>>()
/// );
///
/// // parse multiple values
/// assert_eq!(
///     Ok(vec![80, 443]),
///     config["server"]["port"]
///         .values()
///         .map(|value| value.parse::<u16>())
///         .collect::<Result<Vec<_>, _>>()
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(fuzzing, derive(arbitrary::Arbitrary))]
pub struct Config<'a> {
    pub(crate) quotes: Option<QuoteKind>,
    pub(crate) key: &'a str,
    pub(crate) value: HashMap<&'a str, Config<'a>>,
}

impl Hash for Config<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<'a> Config<'a> {
    pub(crate) fn new(key: &'a str, quotes: Option<QuoteKind>) -> Self {
        Config {
            quotes,
            key,
            value: make_map(),
        }
    }

    pub(crate) fn add_child(&mut self, child: Config<'a>) {
        self.value.insert(child.key, child);
    }

    pub fn quoted(&self) -> bool {
        self.quotes.is_some()
    }

    /// Check whether the config has the node.
    pub fn has_value(&self, value: &str) -> bool {
        self.value.contains_key(value)
    }

    /// Iterator for the children of a node.
    pub fn children(&self) -> impl Iterator<Item = &Config<'a>> {
        self.value.values()
    }

    /// The first child of the node.
    ///
    /// ```
    /// # use nccl::*;
    /// // excerpt of long.nccl:
    /// // strings
    /// //    in which case
    /// //       "just\nuse quotes"
    /// let source = std::fs::read_to_string("examples/long.nccl").unwrap();
    /// let config = parse_config(&source).unwrap();
    /// assert_eq!(
    ///     "just\nuse quotes",
    ///     config["strings"]["in which case:"]
    ///         .child()
    ///         .unwrap()
    ///         .parse_quoted()
    ///         .unwrap()
    /// );
    /// ```
    pub fn child(&self) -> Option<&Config<'a>> {
        self.children().next()
    }

    /// Iterator for the child values of a node.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.value.keys().copied()
    }

    /// The first child value of a node.
    pub fn value(&self) -> Option<&'a str> {
        self.value.iter().next().map(|opt| *opt.0)
    }

    fn pretty_print(&self) -> String {
        self.pp(0)
    }

    fn pp(&self, indent: usize) -> String {
        let mut s = String::new();
        if self.key != TOP_LEVEL_KEY && indent != 0 {
            for _ in 0..indent - 1 {
                s.push_str("    ");
            }
            if let Some(quote) = self.quotes {
                s.push(quote.char());
            }
            s.push_str(self.key);
            if let Some(quote) = self.quotes {
                s.push(quote.char());
            }
            s.push('\n');
        }
        for (_, v) in self.value.iter() {
            s.push_str(&v.pp(indent + 1));
        }
        s
    }

    /// Parse the string including escape sequences if it's quoted.
    ///
    /// Note [`NcclError`] variants produced by this method report the line number as zero. This
    /// behavior is fixed in version 5.1.0. I consider this a non-breaking change because the
    /// current behavior cannot be relied upon for useful logical properties, unless you're using
    /// the zero value produced for some mathematical calculation (in which case I think you
    /// deserve to have your stuff break).
    ///
    /// Operates on the first child of the node. See [`Config::child`].
    pub fn parse_quoted(&self) -> Result<String, NcclError> {
        // TODO use a library for this garbage
        if !self.quoted() {
            Ok(String::from(self.key))
        } else {
            let mut value = Vec::with_capacity(self.key.len());

            let bytes = self.key.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i += 1;
                    if i >= bytes.len() {
                        // TODO get the right start point
                        return Err(NcclError::UnterminatedString { start: 0 });
                    }

                    match bytes[i] {
                        // \n
                        b'n' => {
                            value.push(b'\n');
                            i += 1;
                        }

                        // \r
                        b'r' => {
                            value.push(b'\r');
                            i += 1;
                        }

                        // \\
                        b'\\' => {
                            value.push(b'\\');
                            i += 1;
                        }

                        // \" or \'
                        code @ (b'"' | b'\'') => {
                            value.push(code);
                            i += 1;
                        }

                        // something \
                        //       more stuff
                        b'\r' | b'\n' => {
                            i += 1;

                            if i >= bytes.len() {
                                // TODO get the right start point
                                return Err(NcclError::UnterminatedString { start: 0 });
                            }

                            while bytes[i] == b' ' || bytes[i] == b'\t' {
                                i += 1;

                                if i >= bytes.len() {
                                    // TODO get the right start point
                                    return Err(NcclError::UnterminatedString { start: 0 });
                                }
                            }
                        }

                        _ => {
                            return Err(NcclError::ParseUnknownEscape {
                                escape: bytes[i] as char,
                            });
                        }
                    }
                } else {
                    value.push(bytes[i]);
                    i += 1;
                }
            }

            Ok(String::from_utf8(value)?)
        }
    }
}

impl<'a> Index<&str> for Config<'a> {
    type Output = Config<'a>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.value[index]
    }
}

impl ToString for Config<'_> {
    fn to_string(&self) -> String {
        self.pretty_print()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quoted() {
        let s = "hello\\\n   world";

        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "helloworld"
        );

        let s = "hello \\\n  world";
        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "hello world"
        );

        let s = "hello\\\n\tworld";
        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "helloworld"
        );

        let s = "hello \\\n\tworld";
        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "hello world"
        );

        let s = r#"\"\"\"\""#;
        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "\"\"\"\""
        );

        let s = r#"\'\'\'\'"#;
        assert_eq!(
            Config::new(s, Some(QuoteKind::Single))
                .parse_quoted()
                .unwrap(),
            "''''"
        );

        let s = r#"\\\"#;
        assert!(dbg!(Config::new(s, Some(QuoteKind::Single)).parse_quoted()).is_err());

        let s = "\\\r\t";
        assert!(dbg!(Config::new(s, Some(QuoteKind::Single)).parse_quoted()).is_err());
    }

    #[test]
    fn single_file() {
        let s = std::fs::read_to_string("examples/config.nccl").unwrap();
        let mut c = Config::new(&s[0..3], None);
        c.add_child(Config {
            quotes: None,
            key: &s[3..6],
            value: make_map(),
        });

        assert_eq!(
            c,
            Config {
                quotes: None,
                key: "ser",
                value: {
                    let mut map = make_map();
                    map.insert("ver", Config::new("ver", None));
                    map
                }
            }
        )
    }

    #[test]
    fn multi_file() {
        let s1 = std::fs::read_to_string("examples/config.nccl").unwrap();
        let mut c = Config::new(&s1[0..3], None);

        let s2 = std::fs::read_to_string("examples/config_dos.nccl").unwrap();
        c.add_child(Config {
            quotes: None,
            key: &s2[3..6],
            value: make_map(),
        });

        assert_eq!(
            c,
            Config {
                quotes: None,
                key: "ser",
                value: {
                    let mut map = make_map();
                    map.insert("ver", Config::new("ver", None));
                    map
                }
            }
        )
    }

    #[test]
    fn to_string() {
        let orig_source = std::fs::read_to_string("examples/all-of-em.nccl").unwrap();
        println!("orig\n{}", orig_source);
        let orig_config = crate::parse_config(&orig_source).unwrap();
        println!("{:#?}\n\n\n", orig_config);

        let new_source = orig_config.to_string();
        println!("new\n{}", new_source);
        let new_config = crate::parse_config(&new_source).unwrap();
        println!("{:#?}\n\n\n", new_config);

        assert_eq!(new_config, orig_config);
    }
}

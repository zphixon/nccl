//! Contains the configuration struct

use crate::NcclError;

use std::hash::{Hash, Hasher};
use std::ops::Index;

use indexmap::IndexMap;

/// Type alias for an [`IndexMap`], a hash map where insertion order is preserved.
pub type HashMap<K, V> = IndexMap<K, V, fnv::FnvBuildHasher>;

pub(crate) fn make_map<K, V>() -> HashMap<K, V> {
    HashMap::with_hasher(fnv::FnvBuildHasher::default())
}

/// A nccl configuration
///
/// Index with `&'static str`.
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
pub struct Config<'key, 'value>
where
    'key: 'value,
{
    pub(crate) key: &'key str,
    pub(crate) value: HashMap<&'value str, Config<'value, 'value>>,
}

impl Hash for Config<'_, '_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<'key, 'value> Config<'key, 'value> {
    pub(crate) fn new(key: &'key str) -> Self {
        Config {
            key,
            value: make_map(),
        }
    }

    pub(crate) fn add_child(&mut self, child: Config<'key, 'value>) {
        self.value.insert(child.key, child);
    }

    /// Check whether the config has the node.
    pub fn has_value(&self, value: &str) -> bool {
        self.value.contains_key(value)
    }

    /// Iterator for the children of a node.
    pub fn children(&self) -> impl Iterator<Item = &Config<'value, 'value>> {
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
    pub fn child(&self) -> Option<&Config<'value, 'value>> {
        self.children().nth(0)
    }

    /// Iterator for the child values of a node.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.value.keys().map(|s| *s)
    }

    /// The first child value of a node.
    pub fn value(&self) -> Option<&'value str> {
        self.value.iter().nth(0).map(|opt| *opt.0)
    }

    fn pretty_print(&self) -> String {
        self.pp(0)
    }

    fn pp(&self, indent: usize) -> String {
        let mut s = String::new();
        for _ in 0..indent {
            s.push_str("    ");
        }
        s.push_str(self.key);
        s.push('\n');
        for (_, v) in self.value.iter() {
            s.push_str(&v.pp(indent + 1));
        }
        s
    }

    /// Parse the string including escape sequences if it's quoted.
    ///
    /// See [`Config::child`].
    pub fn parse_quoted(&self) -> Result<String, NcclError> {
        if self.key.starts_with('"') || self.key.starts_with('\'') {
            let mut value = Vec::with_capacity(self.key.len() - 2);

            let bytes = self.key.as_bytes();
            let mut i = 1;

            while i < bytes.len() - 1 {
                if bytes[i] == b'\\' {
                    i += 1;
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
                            while bytes[i] == b' ' || bytes[i] == b'\t' {
                                i += 1;
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
        } else {
            Ok(self.key.to_string())
        }
    }
}

impl<'a> Index<&str> for Config<'a, 'a> {
    type Output = Config<'a, 'a>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.value[index]
    }
}

impl ToString for Config<'_, '_> {
    fn to_string(&self) -> String {
        self.pretty_print()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quoted() {
        let s = "'hello\\\n   world'";

        assert_eq!(Config::new(s).parse_quoted().unwrap(), "helloworld");

        let s = "'hello \\\n  world'";
        assert_eq!(Config::new(s).parse_quoted().unwrap(), "hello world");

        let s = "'hello\\\n\tworld'";
        assert_eq!(Config::new(s).parse_quoted().unwrap(), "helloworld");

        let s = "'hello \\\n\tworld'";
        assert_eq!(Config::new(s).parse_quoted().unwrap(), "hello world");

        let s = r#"'""""'"#;
        assert_eq!(Config::new(s).parse_quoted().unwrap(), "\"\"\"\"");

        let s = r#""''''""#;
        assert_eq!(Config::new(s).parse_quoted().unwrap(), "''''");
    }

    #[test]
    fn single_file() {
        let s = std::fs::read_to_string("examples/config.nccl").unwrap();
        let mut c = Config::new(&s[0..3]);
        c.add_child(Config {
            key: &s[3..6],
            value: make_map(),
        });

        assert_eq!(
            c,
            Config {
                key: "ser",
                value: {
                    let mut map = make_map();
                    map.insert("ver", Config::new("ver"));
                    map
                }
            }
        )
    }

    #[test]
    fn multi_file() {
        let s1 = std::fs::read_to_string("examples/config.nccl").unwrap();
        let mut c = Config::new(&s1[0..3]);

        let s2 = std::fs::read_to_string("examples/config_dos.nccl").unwrap();
        c.add_child(Config {
            key: &s2[3..6],
            value: make_map(),
        });

        assert_eq!(
            c,
            Config {
                key: "ser",
                value: {
                    let mut map = make_map();
                    map.insert("ver", Config::new("ver"));
                    map
                }
            }
        )
    }
}

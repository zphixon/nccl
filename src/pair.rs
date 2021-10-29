use crate::error::{ErrorKind, NcclError};
use crate::value::Value;

use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

use indexmap::IndexMap;

pub type HashMap<K, V> = IndexMap<K, V, fnv::FnvBuildHasher>;

pub(crate) fn make_map<K, V>() -> HashMap<K, V> {
    HashMap::with_hasher(fnv::FnvBuildHasher::default())
}

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
    pub fn new(key: &'key str) -> Self {
        Config {
            key,
            value: make_map(),
        }
    }

    pub(crate) fn add_child(&mut self, child: Config<'key, 'value>) {
        self.value.insert(child.key, child);
    }

    pub fn has_value(&self, value: &str) -> bool {
        self.value.contains_key(value)
    }

    pub fn children(&self) -> impl Iterator<Item = &Config<'value, 'value>> {
        self.value.values()
    }

    pub fn child(&self) -> Option<&Config<'value, 'value>> {
        self.children().nth(0)
    }

    pub fn values(&self) -> impl Iterator<Item = &str> {
        self.value.keys().map(|s| *s)
    }

    pub fn value(&self) -> Option<&'value str> {
        self.value.iter().nth(0).map(|opt| *opt.0)
    }

    pub fn pretty_print(&self) -> String {
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
                            return Err(NcclError::new(
                                ErrorKind::Parse,
                                &format!("Unknown format code: {:?}", bytes[i] as char),
                                0,
                            ))
                        }
                    }
                } else {
                    value.push(bytes[i]);
                    i += 1;
                }
            }

            String::from_utf8(value)
                .map_err(|err| NcclError::new(ErrorKind::Utf8 { err }, "invalid utf8", 0))
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

/// Struct that contains configuration information.
///
/// Examples:
///
/// ```
/// let p = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = p["server"]["port"].keys_as::<i64>().unwrap();
///
/// println!("Operating on ports:");
/// for port in ports.iter() {
///     println!("  {}", port);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    key: Value,
    value: Vec<Pair>,
}

impl Pair {
    /// Creates a new Pair.
    pub fn new<T: Into<Value>>(key: T) -> Self {
        Pair {
            key: key.into(),
            value: vec![],
        }
    }

    /// Adds a value to a Pair.
    ///
    /// Examples:
    ///
    /// ```
    /// let mut p = nccl::Pair::new("hello");
    /// p.add(true);
    /// p.add("world");
    /// ```
    pub fn add<T: Into<Value>>(&mut self, value: T) {
        self.value.push(Pair::new(value.into()));
    }

    /// Recursively adds a slice to a Pair.
    pub fn add_slice(&mut self, path: &[Value]) {
        let s = self.traverse_path(&path[0..path.len() - 1]);
        if !s.has_key(&path[path.len() - 1]) || s[&path[path.len() - 1]].value.is_empty() {
            s.add(&path[path.len() - 1]);
        }
    }

    /// Adds a Pair to a Pair.
    pub fn add_pair(&mut self, pair: Pair) {
        if !self.has_key(&pair.key) {
            self.value.push(pair);
        } else {
            self[&pair.key].value = pair.value;
        }
    }

    /// Test if a pair has a key.
    ///
    /// Examples:
    ///
    /// ```
    /// use nccl::NcclError;
    /// let mut p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert!(p.has_key("server"));
    /// assert!(p["server"]["port"].has_key(80));
    /// ```
    pub fn has_key<T>(&self, key: T) -> bool
    where
        Value: From<T>,
    {
        let k = key.into();
        for item in &self.value {
            if item.key == k {
                return true;
            }
        }

        false
    }

    /// Test if a pair has a path of values. Use `vec_into!` to make
    /// this method easier to use.
    ///
    /// Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate nccl; fn main() {
    /// let mut p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert!(p.has_path(vec_into!["server", "port", 80]));
    /// # }
    /// ```
    pub fn has_path(&self, path: Vec<Value>) -> bool {
        if path.is_empty() {
            true
        } else if self.has_key(path[0].clone()) {
            self[path[0].clone()].has_path(path[1..path.len()].to_vec())
        } else {
            false
        }
    }

    /// Traverses a Pair using a slice, adding the item if it does not exist.
    pub fn traverse_path(&mut self, path: &[Value]) -> &mut Pair {
        if path.is_empty() {
            self
        } else {
            if !self.has_key(&path[0]) {
                self.add(&path[0]);
            }
            self.get(&path[0])
                .unwrap()
                .traverse_path(&path[1..path.len()])
        }
    }

    /// Gets a child Pair from a Pair. Used by Pair's implementation of Index.
    ///
    /// ```
    /// let mut p = nccl::Pair::new("top_level");
    /// p.add("hello!");
    /// p.get("hello!").unwrap();
    /// ```
    pub fn get<T>(&mut self, value: T) -> Result<&mut Pair, NcclError>
    where
        Value: From<T>,
    {
        let v = value.into();

        if self.value.is_empty() {
            return Err(NcclError::new(
                ErrorKind::KeyNotFound,
                &format!("Pair does not have key: {}", v),
                0,
            ));
        } else {
            for item in &mut self.value {
                if item.key == v {
                    return Ok(item);
                }
            }
        }

        Err(NcclError::new(
            ErrorKind::KeyNotFound,
            &format!("Could not find key: {}", v),
            0,
        ))
    }

    /// Gets a mutable child Pair from a Pair. Used by Pair's implementation of
    /// IndexMut.
    ///
    /// ```
    /// let mut p = nccl::Pair::new("top_level");
    /// p.add(32);
    /// p.get(32).unwrap();
    /// ```
    pub fn get_ref<T>(&self, value: T) -> Result<&Pair, NcclError>
    where
        Value: From<T>,
    {
        let v = value.into();

        if self.value.is_empty() {
            return Ok(self);
        } else {
            for item in &self.value {
                if item.key == v {
                    return Ok(item);
                }
            }
        }

        Err(NcclError::new(
            ErrorKind::KeyNotFound,
            &format!("Could not find key: {}", v),
            0,
        ))
    }

    /// Returns the value of a pair as a string. Returns `None` if the pair
    /// is not a leaf.
    /// ```
    /// let config = nccl::parse_file("examples/long.nccl").unwrap();
    /// assert_eq!(config["bool too"].value().unwrap(), "false");
    /// ```
    pub fn value(&self) -> Option<String> {
        if self.value.len() == 1 {
            Some(format!("{}", self.value[0].key.clone()))
        } else {
            None
        }
    }

    /// Returns the value of the key or a default value.
    pub fn value_or(&self, or: String) -> String {
        self.value().unwrap_or(or)
    }

    fn value_raw(&self) -> Option<Value> {
        if self.value.len() == 1 {
            Some(self.value[0].key.clone())
        } else {
            None
        }
    }

    /// Gets the value of a key as a specified type, if there is only one.
    ///
    /// Examples:
    ///
    /// ```
    /// let p = nccl::parse_file("examples/long.nccl").unwrap();
    /// assert!(!p["bool too"].value_as::<bool>().unwrap());
    /// ```
    pub fn value_as<T>(&self) -> Result<T, NcclError>
    where
        Value: TryInto<T>,
    {
        match self.value_raw() {
            Some(v) => match v.try_into() {
                Ok(t) => Ok(t),
                Err(_) => Err(NcclError::new(ErrorKind::Into, "Could not convert to T", 0)),
            },
            None => Err(NcclError::new(
                ErrorKind::MultipleValues,
                "Could not convert value: multiple values. Use keys() or keys_as()",
                0,
            )),
        }
    }

    /// Gets the value of a key as a specified type or a default value.
    pub fn value_as_or<T>(&self, or: T) -> T
    where
        Value: TryInto<T>,
    {
        self.value_as::<T>().unwrap_or(or)
    }

    fn keys(&self) -> Vec<Value> {
        self.value.clone().into_iter().map(|x| x.key).collect()
    }

    /// Gets keys of a value as a vector of T.
    ///
    /// Examples:
    ///
    /// ```
    /// let config = nccl::parse_file("examples/config.nccl").unwrap();
    /// let ports = config["server"]["port"].keys_as::<i64>().unwrap();
    /// assert_eq!(ports, vec![80, 443]);
    /// ```
    pub fn keys_as<T>(&self) -> Result<Vec<T>, NcclError>
    where
        Value: TryInto<T>,
    {
        let mut v: Vec<T> = vec![];
        for key in self.keys() {
            match key.try_into() {
                Ok(k) => v.push(k),
                Err(_) => return Err(NcclError::new(ErrorKind::Into, "Could not convert to T", 0)),
            }
        }
        Ok(v)
    }

    /// Gets keys of a value as a vector of T or returns a default vector.
    pub fn keys_as_or<T>(&self, or: Vec<T>) -> Vec<T>
    where
        Value: TryInto<T>,
    {
        self.keys_as::<T>().unwrap_or(or)
    }

    /// Pretty-prints a Pair.
    ///
    /// Examples:
    ///
    /// ```
    /// let config = nccl::parse_file("examples/config.nccl").unwrap();
    /// config.pretty_print();
    ///
    /// // String("__top_level__")
    /// //     String("server")
    /// //         String("domain")
    /// //             String("example.com")
    /// //             String("www.example.com")
    /// //         String("port")
    /// //             Integer(80)
    /// //             Integer(443)
    /// //         String("root")
    /// //             String("/var/www/html")
    /// ```
    ///
    pub fn pretty_print(&self) {
        self.pp_rec(0);
    }

    fn pp_rec(&self, indent: u32) {
        for _ in 0..indent {
            print!("    ");
        }
        println!("{:?}", self.key);
        for value in &self.value {
            value.pp_rec(indent + 1);
        }
    }
}

impl<T> Index<T> for Pair
where
    Value: From<T>,
{
    type Output = Pair;
    fn index(&self, i: T) -> &Pair {
        self.get_ref(i).unwrap()
    }
}

impl<T> IndexMut<T> for Pair
where
    Value: From<T>,
{
    fn index_mut(&mut self, i: T) -> &mut Pair {
        self.get(i).unwrap()
    }
}

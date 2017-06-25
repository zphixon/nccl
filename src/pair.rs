
use error::{NcclError, ErrorKind};

use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::error::Error;

/// Struct that contains configuration information.
///
/// Examples:
///
/// ```
/// let p = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = p["server"]["port"].keys_as::<u32>().unwrap();
///
/// println!("Operating on ports:");
/// for port in ports.iter() {
///     println!("  {}", port);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    key: String,
    value: Vec<Pair>
}

impl Pair {
    /// Creates a new Pair.
    pub fn new(key: &str) -> Self {
        Pair {
            key: key.to_owned(),
            value: vec![]
        }
    }

    /// Adds a value to a Pair.
    ///
    /// Examples:
    ///
    /// ```
    /// let mut p = nccl::Pair::new("hello");
    /// p.add("world");
    /// ```
    pub fn add(&mut self, value: &str) {
        self.value.push(Pair::new(value));
    }

    /// Recursively adds a slice to a Pair.
    pub fn add_slice(&mut self, path: &[String]) {
        let mut s = self.traverse_path(&path[0..path.len() - 1]);
        if !s.has_key(&path[path.len() - 1]) {
            s.add(&path[path.len() - 1]);
        }
    }

    /// Adds a Pair to a Pair.
    pub fn add_pair(&mut self, pair: Pair) {
        if !self.keys().contains(&pair.key) {
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
    /// let p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert!(p.has_key("server"));
    /// ```
    pub fn has_key(&self, key: &str) -> bool {
        for item in self.value.iter() {
            if &item.key == key {
                return true;
            }
        }

        return false;
    }

    /// Traverses a Pair using a slice, adding the item if it does not exist.
    pub fn traverse_path(&mut self, path: &[String]) -> &mut Pair {
        if path.len() == 0 {
            self
        } else {
            if !self.has_key(&path[0]) {
                self.add(&path[0]);
            }
            self.get(&path[0]).unwrap().traverse_path(&path[1..path.len()])
        }
    }

    /// Gets a child Pair from a Pair. Used by Pair's implementation of Index.
    ///
    /// ```
    /// let mut p = nccl::Pair::new("top_level");
    /// p.add("hello!");
    /// p.get("hello!").unwrap();
    /// ```
    pub fn get(&mut self, value: &str) -> Result<&mut Pair, Box<Error>> {
        let value = value.to_owned();

        if self.value.is_empty() {
            return Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Pair does not have key: {}", value), 0)));
        } else {
            for item in self.value.iter_mut() {
                if item.key == value {
                    return Ok(item);
                }
            }
        }

        Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Could not find key: {}", value), 0)))
    }

    /// Gets a mutable child Pair from a Pair. Used by Pair's implementation of
    /// IndexMut.
    ///
    /// ```
    /// let mut p = nccl::Pair::new("top_level");
    /// p.add("hello!");
    /// p.get("hello!").unwrap();
    /// ```
    pub fn get_ref(&self, value: &str) -> Result<&Pair, Box<Error>> {
        let value_owned = value.to_owned();

        if self.value.is_empty() {
            return Ok(self);
        } else {
            for item in self.value.iter() {
                if item.key == value_owned {
                    return Ok(item);
                }
            }
        }

        Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, "Cound not find key", 0)))
    }

    /// Gets the value of a key if there is only one.
    ///
    /// Examples:
    ///
    /// ```
    /// let p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert_eq!(p["server"]["root"].value().unwrap(), "/var/www/html");
    /// ```
    pub fn value(&self) -> Option<String>  {
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
    /// assert_eq!(p["bool too"].value_as::<bool>().unwrap(), false);
    /// ```
    pub fn value_as<T>(&self) -> Result<T, Box<Error>> where T: FromStr {
        match self.value() {
            Some(value) => match value.parse::<T>() {
                Ok(ok) => Ok(ok),
                Err(_) => Err(Box::new(NcclError::new(ErrorKind::ParseError, "Could not parse value", 0)))
            },
            None => Err(Box::new(NcclError::new(ErrorKind::NoValue, "Key has no or multiple associated values", 0)))
        }
    }

    /// Gets keys of a value as a vector of Strings.
    ///
    /// Examples:
    ///
    /// ```
    /// let v: Vec<String> = vec!["bologne".into(), "ham".into()];
    /// let p = nccl::parse_file("examples/inherit.nccl").unwrap();
    /// assert_eq!(p["sandwich"]["meat"].keys(), v);
    /// ```
    pub fn keys(&self) -> Vec<String> {
        self.value.clone().into_iter().map(|x| x.key).collect()
    }

    /// Gets keys of a value as a vector of T.
    ///
    /// Examples:
    ///
    /// ```
    /// let config = nccl::parse_file("examples/config.nccl").unwrap();
    /// let ports = config["server"]["port"].keys_as::<i32>().unwrap();
    /// assert_eq!(ports, vec![80, 443]);
    /// ```
    pub fn keys_as<T: FromStr>(&self) -> Result<Vec<T>, Box<Error>> {
        // XXX this is gross
        match self.keys().iter().map(|s| s.parse::<T>()).collect::<Result<Vec<T>, _>>() {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Box::new(NcclError::new(ErrorKind::FromStrError, "Could not parse keys", 0)))
        }
    }

    /// Pretty-prints a Pair.
    pub fn pretty_print(&self) {
        self.pp_rec(0);
    }

    fn pp_rec(&self, indent: u32) {
        for _ in 0..indent {
            print!("    ");
        }
        println!("{}", self.key);
        for value in self.value.iter() {
            value.pp_rec(indent + 1);
        }
    }
}

impl<'a> Index<&'a str> for Pair {
    type Output = Pair;
    fn index(&self, i: &str) -> &Pair {
        self.get_ref(i).unwrap()
    }
}

impl<'a> IndexMut<&'a str> for Pair {
    fn index_mut(&mut self, i: &str) -> &mut Pair {
        self.get(i).unwrap()
    }
}


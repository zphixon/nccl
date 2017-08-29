
use error::{NcclError, ErrorKind};
use value::Value;
use ::TryInto;

use std::ops::{Index, IndexMut};
use std::error::Error;

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
    value: Vec<Pair>
}

impl Pair {
    /// Creates a new Pair.
    pub fn new<T: Into<Value>>(key: T) -> Self {
        Pair {
            key: key.into(),
            value: vec![]
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
        if !s.has_key(&path[path.len() - 1]) {
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
    pub fn has_key<T>(&self, key: T) -> bool where Value: From<T> {
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
        if path.len() == 0 {
            true
        } else {
            if self.has_key(path[0].clone()) {
                self[path[0].clone()].has_path(path[1..path.len()].to_vec())
            } else {
                false
            }
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
    pub fn get<T>(&mut self, value: T) -> Result<&mut Pair, Box<Error>> where Value: From<T> {
        let v = value.into();

        if self.value.is_empty() {
            return Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Pair does not have key: {}", v), 0)));
        } else {
            for item in &mut self.value {
                if item.key == v {
                    return Ok(item);
                }
            }
        }

        Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Could not find key: {}", v), 0)))
    }

    /// Gets a mutable child Pair from a Pair. Used by Pair's implementation of
    /// IndexMut.
    ///
    /// ```
    /// let mut p = nccl::Pair::new("top_level");
    /// p.add(32);
    /// p.get(32).unwrap();
    /// ```
    pub fn get_ref<T>(&self, value: T) -> Result<&Pair, Box<Error>> where Value: From<T> {
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

        Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, "Cound not find key", 0)))
    }

    fn value(&self) -> Option<Value>  {
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
    pub fn value_as<T>(&self) -> Result<T, Box<Error>> where Value: TryInto<T> {
        match self.value() {
            Some(v) => match v.try_into() {
                Ok(t) => Ok(t),
                Err(_) => return Err(Box::new(NcclError::new(ErrorKind::IntoError, "Could not convert to T", 0)))
            },
            None => Err(Box::new(NcclError::new(ErrorKind::MultipleValues, "Could not convert value: multiple values. Use keys() or keys_as()", 0)))
        }
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
    pub fn keys_as<T>(&self) -> Result<Vec<T>, Box<Error>> where Value: TryInto<T> {
        let mut v: Vec<T> = vec![];
        for key in self.keys() {
            match key.try_into() {
                Ok(k) => v.push(k),
                Err(_) => return Err(Box::new(NcclError::new(ErrorKind::IntoError, "Could not convert to T", 0)))
            }
        }
        Ok(v)
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
        println!("{}", self.key);
        for value in &self.value {
            value.pp_rec(indent + 1);
        }
    }
}

impl<T> Index<T> for Pair where Value: From<T> {
    type Output = Pair;
    fn index(&self, i: T) -> &Pair {
        self.get_ref(i).unwrap()
    }
}

impl<T> IndexMut<T> for Pair where Value: From<T> {
    fn index_mut(&mut self, i: T) -> &mut Pair {
        self.get(i).unwrap()
    }
}



use std::ops::{Index, IndexMut};
use error::Error;

// top level key that contains everything is __top_level__
#[derive(Clone, Debug, PartialEq)]
/// Structure that represents a key-value pair.
pub struct Pair {
    pub key: String,
    pub value: Vec<Pair>
}

impl Pair {
    /// Creates a new Pair using a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// let p1 = Pair::new("Hello!");
    /// ```
    ///
    pub fn new(key: &str) -> Pair {
        Pair {
            key: key.to_owned(),
            value: vec![]
        }
    }

    /// Creates a new Pair using a `String`
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// let p = Pair::new_string("Hello!".into());
    /// ```
    pub fn new_string(key: String) -> Pair {
        Pair {
            key: key,
            value: vec![]
        }
    }

    /// Adds a value to a Pair. Overwrites values if they exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// let mut p = Pair::new("top");
    /// p.add("hello");
    /// p.add("6.28");
    /// ```
    ///
    /// `add` overwrites values if they exist:
    ///
    /// ```
    /// # use nccl::Pair;
    /// let mut p = Pair::new("top");
    /// p.add("hello");
    /// p["hello"].add("everyone");
    /// p.add("hello");
    ///
    /// assert_eq!(p["hello"], Pair {
    ///     key: "hello".into(),
    ///     value: vec![]
    /// });
    /// ```
    ///
    pub fn add(&mut self, val: &str) {
        self.add_pair(Pair::new(val));
    }

    /// Adds a value to a Pair. Overwrites values if they exist.
    pub fn add_string(&mut self, val: String) {
        self.add_pair(Pair::new_string(val));
    }

    /// Adds a Pair to a Pair. Overwrites values if they exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// // create a new Pair
    /// let mut p1 = Pair::new("hi");
    ///
    /// // add some stuff to the pair
    /// p1.add("people");
    /// p1.add("friends");
    ///
    /// // we want to say hello some more
    /// let mut p2 = Pair::new("friends");
    ///
    /// // add some friends
    /// p2.add("John");
    /// p2.add("Alex");
    ///
    /// // say hello to our friends
    /// p1.add_pair(p2);
    /// ```
    ///
    /// `add_pair` overwrites values if they exist:
    ///
    /// ```
    /// # use nccl::Pair;
    /// // create a new pair
    /// let mut p1 = Pair::new("happy birthday");
    ///
    /// // we think Ron's birthday is the 3rd...
    /// p1.add("Ron");
    /// p1["Ron"].add("March 3rd");
    ///
    /// // whoops, we were wrong
    /// let mut p2 = Pair::new("Ron");
    /// p2.add("March 2nd");
    ///
    /// // there you go Ron, happy belated birthday
    /// p1.add_pair(p2);
    /// ```
    pub fn add_pair(&mut self, pair: Pair) {
        if !self.value.contains(&pair) {
            self.value.push(pair);
        } else {
            self[&pair.key].value = pair.value;
        }
    }

    /// Gets an immutable reference to a Pair in a Pair.
    pub fn get_pair(&self, value: String) -> Result<&Pair, Error> {
        let mut p = None;
        for (k, v) in self.value.iter().enumerate() {
            if v.key == value {
                p = Some(k);
            }
        }
        if p.is_some() {
            Ok(&self.value[p.unwrap()])
        } else {
            Err(Error::KeyNotFound)
        }
    }

    /// Gets a mutable reference to a Pair in a Pair.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// let mut p = Pair::new("heyo");
    /// p.add("waddup");
    /// p["waddup"].add("my pal");
    ///
    /// // indexing calls get/get_mut under the hood
    /// p.get_pair_mut("waddup".into()).unwrap().add("friend");
    /// // equivalent to
    /// p["waddup"].add("friend");
    /// ```
    pub fn get_pair_mut(&mut self, value: String) -> Result<&mut Pair, Error> {
        let mut p = None;
        for (k, v) in self.value.iter().enumerate() {
            if v.key == value {
                p = Some(k);
            }
        }
        if p.is_some() {
            Ok(&mut self.value[p.unwrap()])
        } else {
            Err(Error::KeyNotFound)
        }
    }

    /// Gets the value associated with a pair.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::Pair;
    /// let mut p = Pair::new("numbers");
    /// p.add("one");
    /// p.add("two");
    /// p["one"].add("true");
    /// p["two"].add("false");
    /// assert_eq!(p["one"].get(), "true");
    /// assert_eq!(p["two"].get(), "false");
    /// ```
    pub fn get(&self) -> &String {
        if self.value.len() == 0 {
            panic!("No value associated with key");
        }
        &self.value[0].key
    }
}

impl<'a> Index<&'a str> for Pair {
    type Output = Pair;
    fn index(&self, value: &'a str) -> &Pair {
        self.get_pair(value.to_owned()).expect("Did not find value in pair")
    }
}

impl<'a> IndexMut<&'a str> for Pair {
    fn index_mut(&mut self, value: &'a str) -> &mut Pair {
        self.get_pair_mut(value.to_owned()).expect("Did not find value in pair")
    }
}


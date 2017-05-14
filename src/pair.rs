
use std::ops::{Index, IndexMut};
use value::Value;
use error::Error;

// top level key that contains everything is __top_level__
#[derive(Clone, Debug, PartialEq)]
/// Structure that represents a key-value pair.
pub struct Pair {
    pub key: Value,
    pub value: Vec<Pair>
}

impl Pair {
    /// Creates a new Pair.
    ///
    /// # Examples
    ///
    /// Any type where [`Value`] implements `From` can be supplied:
    ///
    /// ```
    /// # use nccl::pair::Pair;
    /// let p1 = Pair::new("Hello!");
    /// let p2 = Pair::new(true);
    /// // etc
    /// ```
    ///
    /// [`Value`]: ../value/enum.Value.html
    pub fn new<T>(key: T) -> Pair where Value: From<T> {
        Pair {
            key: Value::from(key),
            value: vec![]
        }
    }

    /// Adds a [`Value`] to a Pair. Overwrites values if they exist.
    ///
    /// # Examples
    ///
    /// Any type where [`Value`] implements `From` can be supplied:
    ///
    /// ```
    /// # use nccl::pair::Pair;
    /// let mut p = Pair::new("top");
    /// p.add("hello");
    /// p.add(6.28);
    /// ```
    ///
    /// `add` overwrites values if they exist:
    ///
    /// ```
    /// # use nccl::pair::Pair;
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
    /// [`Value`]: ../value/enum.Value.html
    pub fn add<T>(&mut self, val: T) where Value: From<T> {
        self.add_pair(Pair::new(val));
    }

    /// Adds a Pair to a Pair. Overwrites values if they exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nccl::pair::Pair;
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
    /// # use nccl::pair::Pair;
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
            self[pair.key].value = pair.value;
        }
    }

    /// Gets an immutable reference to a Pair in a Pair.
    pub fn get(&self, value: Value) -> Result<&Pair, Error> {
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
    /// # use nccl::pair::Pair;
    /// let mut p = Pair::new("heyo");
    /// p.add("waddup");
    /// p["waddup"].add("my pal");
    ///
    /// // indexing calls get/get_mut under the hood
    /// p.get_mut("waddup".into()).unwrap().add("friend");
    /// // equivalent to
    /// p["waddup"].add("friend");
    /// ```
    pub fn get_mut(&mut self, value: Value) -> Result<&mut Pair, Error> {
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
}

impl<T> Index<T> for Pair where Value: From<T> {
    type Output = Pair;
    fn index(&self, value: T) -> &Pair {
        self.get(value.into()).expect("Did not find value in pair")
    }
}

impl<T> IndexMut<T> for Pair where Value: From<T> {
    fn index_mut(&mut self, value: T) -> &mut Pair {
        self.get_mut(value.into()).expect("Did not find value in pair")
    }
}

impl Index<Pair> for Pair {
    type Output = Pair;
    fn index(&self, value: Pair) -> &Pair {
        self.get(value.key).expect("Did not find value in pair")
    }
}

impl IndexMut<Pair> for Pair {
    fn index_mut(&mut self, value: Pair) -> &mut Pair {
        self.get_mut(value.key).expect("Did not find value in pair")
    }
}


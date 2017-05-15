
use std::ops::{Index, IndexMut, Deref};
use error::Error;

// top level key that contains everything is __top_level__
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    pub key: String,
    pub value: Vec<Pair>
}

impl Pair {
    pub fn new(key: &str) -> Pair {
        Pair {
            key: key.to_owned(),
            value: vec![]
        }
    }

    pub fn new_string(key: String) -> Pair {
        Pair {
            key: key,
            value: vec![]
        }
    }

    pub fn add(&mut self, val: &str) {
        self.add_pair(Pair::new(val));
    }

    pub fn add_string(&mut self, val: String) {
        self.add_pair(Pair::new_string(val));
    }

    pub fn add_pair(&mut self, pair: Pair) {
        if !self.value.contains(&pair) {
            self.value.push(pair);
        } else {
            self[&pair.key].value = pair.value;
        }
    }

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

impl Deref for Pair {
    type Target = String;
    fn deref(&self) -> &String {
        &self.get()
    }
}


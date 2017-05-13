
use std::ops::{Index, IndexMut};
use value::Value;
use error::Error;

// top level key that contains everything is __top_level__
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    pub key: Value,
    pub value: Vec<Pair>
}

impl Pair {
    pub fn new<T>(key: T) -> Pair where Value: From<T> {
        Pair {
            key: Value::from(key),
            value: vec![]
        }
    }

    pub fn add<T>(&mut self, val: T) where Value: From<T> {
        self.value.push(Pair::new(val));
    }

    pub fn add_pair(&mut self, p: Pair) {
        self.value.push(p);
    }

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

impl Index<Value> for Pair {
    type Output = Pair;

    fn index(&self, value: Value) -> &Pair {
        self.get(value).expect("Did not find value in pair")
    }
}

impl IndexMut<Value> for Pair {
    fn index_mut(&mut self, value: Value) -> &mut Pair {
        self.get_mut(value).expect("Did not find value in pair")
    }
}



use std::ops::{Index, IndexMut};
use value::Value;

// top level key that contains everything is __top_level__
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    pub key: String,
    pub value: Vec<Pair>
}

impl Pair {
    pub fn new(key: &str) -> Self {
        Pair {
            key: key.to_owned(),
            value: vec![]
        }
    }

    pub fn add(&mut self, value: &str) {
        self.value.push(Pair::new(value));
    }

    pub fn has_key(&self, key: &str) -> bool {
        for item in self.value.iter() {
            if &item.key == key {
                return true;
            }
        }
        return false;
    }

    pub fn get(&mut self, value: &str) -> &mut Pair {
        let value_owned = value.to_owned();
        if self.value.is_empty() {
            return self;
        } else {
            for item in self.value.iter_mut() {
                if item.key == value_owned {
                    return item;
                }
            }
        }
        panic!("\"{}\" not found in pair", value_owned);
    }

    fn get_ref(&self, value: &str) -> &Pair {
        let value_owned = value.to_owned();
        if self.value.is_empty() {
            return self;
        } else {
            for item in self.value.iter() {
                if item.key == value_owned {
                    return item;
                }
            }
        }
        panic!("\"{}\" not found in pair", value_owned);
    }

    pub fn value(&self) -> Value {
        if self.value.len() == 1 {
            return Value::String(self.value[0].key.clone());
        } else if self.value.len() > 1 {
            return Value::Vec(self.value.clone());
        }
        panic!("pair is terminal");
    }
}

impl<'a> Index<&'a str> for Pair {
    type Output = Pair;
    fn index(&self, i: &str) -> &Pair {
        self.get_ref(i)
    }
}

impl<'a> IndexMut<&'a str> for Pair {
    fn index_mut(&mut self, i: &str) -> &mut Pair {
        self.get(i)
    }
}



use std::ops::{Index, IndexMut};
use std::str::FromStr;

use error::{NcclError, ErrorKind};
use token::{TokenKind, Token};

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

    pub fn add(&mut self, value: &str) {
        self.value.push(Pair::new(value));
    }

    pub fn add_slice(&mut self, path: &[String]) {
        let mut s = self.traverse_path(&path[0..path.len() - 1]);
        if !s.has_key(&path[path.len() - 1]) {
            s.add(&path[path.len() - 1]);
        }
    }

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

    pub fn add_pair(&mut self, pair: Pair) {
        if !self.keys().contains(&pair.key) {
            self.value.push(pair);
        } else {
            self[&pair.key].value = pair.value;
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        for item in self.value.iter() {
            if &item.key == key {
                return true;
            }
        }

        return false;
    }

    pub fn get(&mut self, value: &str) -> Result<&mut Pair, NcclError> {
        let value = value.to_owned();

        if self.value.is_empty() {
            return Err(NcclError::new(ErrorKind::KeyNotFound, &format!("Pair does not have key: {}", value), 0));
        } else {
            for item in self.value.iter_mut() {
                if item.key == value {
                    return Ok(item);
                }
            }
        }

        Err(NcclError::new(ErrorKind::KeyNotFound, &format!("Could not find key: {}", value), 0))
    }

    pub fn get_ref(&self, value: &str) -> Result<&Pair, NcclError> {
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

        Err(NcclError::new(ErrorKind::KeyNotFound, "Cound not find key", 0))
    }

    pub fn value(&self) -> Result<String, NcclError>  {
        if self.value.len() == 1 {
            Ok(self.value[0].key.clone())
        } else if self.value.len() > 1 {
            Err(NcclError::new(ErrorKind::NoValue, "Key has multiple values", 0))
        } else {
            Err(NcclError::new(ErrorKind::NoValue, "Key does not have value", 0))
        }
    }

    pub fn value_as<T>(&self) -> Result<T, NcclError> where T: FromStr {
        match self.value() {
            Ok(value) => match value.parse::<T>() {
                Ok(ok) => Ok(ok),
                Err(_) => Err(NcclError::new(ErrorKind::ParseError, "Could not parse value", 0))
            },
            Err(err) => Err(err)
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.value.clone().into_iter().map(|x| x.key).collect()
    }

    pub fn keys_as<T: FromStr>(&self) -> Result<Vec<T>, NcclError> {
        // XXX this is gross
        match self.keys().iter().map(|s| s.parse::<T>()).collect::<Result<Vec<T>, _>>() {
            Ok(ok) => Ok(ok),
            Err(_) => Err(NcclError::new(ErrorKind::FromStrError, "Could not parse keys", 0))
        }
    }

    pub fn as_tokens(&self) -> Vec<Token> {
        self.as_tokens_rec(0)
    }

    fn as_tokens_rec(&self, indent: u32) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        tokens.push(Token::new(TokenKind::Value, self.key.clone(), 0));
        tokens.push(Token::new(TokenKind::Newline, "\n".into(), 0));
        for value in self.value.iter() {
            for _ in 0..indent {
                tokens.push(Token::new(TokenKind::Indent, "".into(), 0));
            }
            tokens.append(&mut value.as_tokens_rec(indent + 1));
            tokens.push(Token::new(TokenKind::Newline, "\n".into(), 0));
        }
        tokens
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


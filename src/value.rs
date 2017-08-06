
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl<'a> From<&'a Value> for Value {
    fn from(v: &'a Value) -> Self {
        *v
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<'a> From<&'a String> for Value {
    fn from(s: &'a String) -> Self {
        Value::String(s.to_owned())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl<'a> Into<&'a Value> for Value {
    fn into(self) -> &'a Self {
        &self
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("value is not a string: {}", self)
        }
    }
}

impl<'a> Into<&'a String> for Value {
    fn into(self) -> &'a String {
        match self {
            Value::String(s) => &s,
            _ => panic!("value is not a string: {}", self)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


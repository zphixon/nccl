
use std::fmt;

pub fn parse_into_value(into: String) -> Value {
    match into.parse::<bool>() {
        Ok(b) => return Value::Bool(b),
        Err(_) => {},
    }

    match into.parse::<i64>() {
        Ok(i) => return Value::Integer(i),
        Err(_) => {},
    }

    match into.parse::<f64>() {
        Ok(f) => return Value::Float(f),
        Err(_) => {},
    }

    Value::String(into)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl<'a> From<&'a Value> for Value {
    fn from(v: &'a Value) -> Self {
        v.clone()
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

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("value is not a string: {}", self)
        }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Value::Bool(b) => b,
            _ => panic!("value is not a bool: {}", self)
        }
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        match self {
            Value::Integer(i) => i,
            _ => panic!("value is not an integer: {}", self)
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::Float(f) => f,
            _ => panic!("value is not a float: {}", self)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


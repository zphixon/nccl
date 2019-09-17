
use ::TryInto;

use std::fmt;

/// Parses a String into a Value, first attempting bool, i64, and f64.
///
/// Examples:
///
/// ```
/// # use nccl::Value; use nccl::parse_into_value;
/// match parse_into_value("32.3".into()) {
///     Value::Float(f) => println!("value is float: {}", f),
///     _ => panic!("it's broke yo"),
/// }
///
/// match parse_into_value("something silly".into()) {
///     Value::String(s) => println!("none of the above: {}", s),
///     _ => panic!("it's really broke yo")
/// }
/// ```
pub fn parse_into_value(into: String) -> Value {
    if let Ok(b) = into.parse::<bool>() {
        return Value::Bool(b);
    }
    if let Ok(i) = into.parse::<i64>() {
        return Value::Integer(i);
    }
    if let Ok(f) = into.parse::<f64>() {
        return Value::Float(f);
    }

    Value::String(into)
}

#[derive(Debug, PartialEq, Clone)]
/// Wrapper type for possible types in nccl configuration.
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl TryInto<String> for Value {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(())
        }
    }
}

impl TryInto<bool> for Value {
    type Error = ();
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(())
        }
    }
}

impl TryInto<i64> for Value {
    type Error = ();
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Integer(i) => Ok(i),
            _ => Err(())
        }
    }
}

impl TryInto<i32> for Value {
    type Error = ();
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Value::Integer(i) => Ok(i as i32),
            _ => Err(())
        }
    }
}

impl TryInto<u64> for Value {
    type Error = ();
    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Value::Integer(i) => Ok(i as u64),
            _ => Err(())
        }
    }
}

impl TryInto<u32> for Value {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Value::Integer(i) => Ok(i as u32),
            _ => Err(())
        }
    }
}

impl TryInto<f64> for Value {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Float(f) => Ok(f),
            _ => Err(())
        }
    }
}

impl TryInto<f32> for Value {
    type Error = ();
    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Value::Float(f) => Ok(f as f32),
            _ => Err(())
        }
    }
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

impl From<u64> for Value {
    fn from(u: u64) -> Self {
        Value::Integer(u as i64)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Value::Integer(i64::from(u))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Float(f64::from(f))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(ref s) => write!(f, "{}", s),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Integer(i) => write!(f, "{}", i),
        }
    }
}


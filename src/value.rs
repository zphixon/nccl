
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    List(Vec<Value>),
    Bool(bool),
    Integer(i64),
    Float(f64),
    Date(String), // TODO
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value {
        Value::String(s.to_owned())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Value {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Value {
        Value::Float(f)
    }
}

impl<T> From<Vec<T>> for Value where Value: From<T> {
    fn from(v: Vec<T>) -> Value {
        Value::List(v.into_iter().map(Value::from).collect())
    }
}


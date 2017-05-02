
//  key
//      value
//  Key {
//      value: Value::String,
//      sub: Value::String
//  }
//
//  key
//      value1
//      value2
//  Key {
//      value: Value::String,
//      sub: Value::List(vec![Value::String, value::String])
//  }
//
//  key
//      value
//          sub
//      value
//  Key {
//      value: Value::String,
//      sub: Value::List(vec![Value::Key(Key {
//                                           value: Value::String
//                                           sub: Value::String
//                                       }),
//                            Value::String])
//  }
//

#[derive(Debug, PartialEq)]
pub struct Key {
    value: Value,
    sub: Value,
}

impl Key {
    fn add(&mut self, v: Value) {
        if self.sub.is_list() {
            self.sub.push(v);
            } else if self.sub.user {
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    List(Vec<Value>),
    Bool(bool),
    Integer(i64),
    Float(f64),
    Date(String), // TODO
    Key(Box<Key>),
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
        Value::List(v.into_iter().map(|x| Value::from(x)).collect())
    }
}

impl Value {
    fn is_list(&v: Value) -> bool {
        match v {
            Value::List(_) => true,
            _ => false
        }
    }

    fn push(&mut self, v: Value) {
        match *self {
            Value::List(l) => l.push(v),
            _ => panic!("Called push on non-List")
        }
    }
}


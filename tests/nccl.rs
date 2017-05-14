
extern crate nccl;

use nccl::value::Value;
use nccl::pair::Pair;

#[test]
fn from_vec() {
    assert_eq!(Value::from(vec![1, 2, 3]),
               Value::List(vec![Value::Integer(1),
                                Value::Integer(2),
                                Value::Integer(3)]));
}

#[test]
fn add() {
    let mut p = Pair::new("key");
    p.add("value");
    assert_eq!(p, Pair {
        key: "key".into(),
        value: vec![Pair {
            key: "value".into(),
            value: vec![]
        }]
    });
}

#[test]
fn get() {
    let mut p = Pair::new("key");
    p.add("value");
    p.add("testaroni");
    p.get_mut("value".into()).unwrap().add("Hello!");
    let v = p.get("value".into()).unwrap();
    assert_eq!(v, &Pair {
        key: "value".into(),
        value: vec![Pair {
            key: "Hello!".into(),
            value: vec![]
        }]
    });
}

#[test]
fn multi() {
    let mut p = Pair::new("key");
    p.add("value");
    p.add("dj khaled");
    p.get_mut("value".into()).unwrap().add("yes!");
    p.get_mut("dj khaled".into()).unwrap().add("you smart");
    p.get_mut("dj khaled".into()).unwrap().add("you loyal");
    assert_eq!(p, Pair {
        key: "key".into(),
        value: vec![Pair {
            key: "value".into(),
            value: vec![Pair {
                key: "yes!".into(),
                value: vec![]
            }]
        }, Pair {
            key: "dj khaled".into(),
            value: vec![Pair {
                key: "you smart".into(),
                value: vec![]
            }, Pair{
                key: "you loyal".into(),
                value: vec![]
            }]
        }]
    });
}

#[test]
fn index() {
    let mut p = Pair::new("key");
    p.add("hello");
    p["hello"].add("world");
    p["hello"]["world"].add("what's");
    p["hello"]["world"]["what's"].add("up?");
    assert_eq!(p, Pair {
        key: "key".into(),
        value: vec![Pair {
            key: "hello".into(),
            value: vec![Pair {
                key: "world".into(),
                value: vec![Pair {
                    key: "what's".into(),
                    value: vec![Pair {
                        key: "up?".into(),
                        value: vec![]
                    }]
                }]
            }]
        }]
    });
}

#[test]
fn index_pair() {
    let mut p1 = Pair::new("aaa");
    p1.add("hello");
    p1["hello"].add("everyone");
    let p2 = Pair::new("hello");
    assert_eq!(p1["hello"], p1[p2]);
}

#[test]
fn index_value() {
    let mut p = Pair::new("charles");
    p.add("fjfj");
    p["fjfj"].add(true);
    assert_eq!(p["fjfj"], p[Value::String("fjfj".into())]);
}

#[test]
fn overwrite_add() {
    let mut p = Pair::new("top");
    p.add("hello");
    p["hello"].add("everyone");
    p.add("hello");
    assert_eq!(p["hello"], Pair {
        key: "hello".into(),
        value: vec![]
    });
}


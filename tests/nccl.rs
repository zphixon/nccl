
extern crate nccl;

use nccl::{Pair, Value};

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
fn get_pair() {
    let mut p = Pair::new("key");
    p.add("value");
    p.add("testaroni");
    p.get_pair_mut("value".into()).unwrap().add("Hello!");
    let v = p.get_pair("value".into()).unwrap();
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
    p.get_pair_mut("value".into()).unwrap().add("yes!");
    p.get_pair_mut("dj khaled".into()).unwrap().add("you smart");
    p.get_pair_mut("dj khaled".into()).unwrap().add("you loyal");
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
    assert_eq!(p["hello"]["world"]["what's"]["up?"], Pair {
        key: "up?".into(),
        value: vec![]
    });
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

#[test]
fn index_get_value() {
    let mut p = Pair::new("hello");
    p.add("one");
    p.add("two");
    p["one"].add(true);
    p["two"].add(false);
    assert_eq!(*p["one"].get(), true.into());
    assert_eq!(*p["two"].get(), false.into());
}


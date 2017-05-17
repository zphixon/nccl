
extern crate nccl;

use nccl::{Pair, Value};

#[test]
fn pair_new() {
    assert_eq!(Pair::new("hello"), Pair {
        key: "hello".into(),
        value: vec![]
    });
}

#[test]
fn pair_add() {
    let mut p = Pair::new("top_level");
    p.add("key");
    assert_eq!(p, Pair {
        key: "top_level".into(),
        value: vec![Pair {
            key: "key".into(),
            value: vec![]
        }]
    });
}

#[test]
fn pair_get() {
    let mut p = Pair::new("top_level");
    p.add("key");
    assert_eq!(p.get("key"), &mut Pair {
        key: "key".into(),
        value: vec![]
    });
}

#[test]
fn pair_index() {
    let mut p = Pair::new("top_level");
    p.add("key");
    assert_eq!(p["key"], Pair {
        key: "key".into(),
        value: vec![]
    });
}

#[test]
fn pair_keys() {
    let mut p = Pair::new("top");
    p.add("numbers");
    p["numbers"].add("1");
    p["numbers"].add("2");
    p["numbers"].add("3");
    p["numbers"].add("4");
    p["numbers"].add("5");
    assert_eq!(p["numbers"].keys(), vec!["1", "2", "3", "4", "5"]);
}

#[test]
fn readme() {
    let mut config = Pair::new("top_level");

    // server
    //     domain
    //         example.com
    //         www.example.com
    //     port
    //         80
    //         443
    //     root
    //         /var/www/html

    config.add("server");
    config["server"].add("domain");
    config["server"].add("port");
    config["server"].add("root");
    config["server"]["domain"].add("example.com");
    config["server"]["domain"].add("www.example.com");
    config["server"]["port"].add("80");
    config["server"]["port"].add("443");
    config["server"]["root"].add("/var/www/html");

    let ports = config["server"]["port"].keys_as::<i32>();
    assert_eq!(ports, vec![80, 443]);
}


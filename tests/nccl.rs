
extern crate nccl;

use nccl::{Pair, Scanner, NcclError, ErrorKind};

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
    assert_eq!(p.get("key").unwrap(), &mut Pair {
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
fn pair_value_parse() {
    let mut p = Pair::new("top");
    p.add("bools");
    p["bools"].add("true");
    assert!(p["bools"].value_as::<bool>().unwrap());
}

#[test]
fn scanner_literal() {
    let mut s = Scanner::new("\"this: is neato\": burrito 64\n    yes.".into());
    s.scan_tokens().unwrap();
}

#[test]
#[should_panic]
fn error_key_not_found() {
    let mut p = Pair::new("jjj");
    p.get("jwiofiojwaef jio").unwrap();
}

#[test]
fn scan_file() {
    assert!(nccl::parse_file("config.nccl").is_ok());
}

#[test]
fn dos_unix_lines() {
    assert_eq!(nccl::parse_file("config.nccl"), nccl::parse_file("config_dos.nccl"));
}

#[test]
fn string_escape() {
    let mut s = Scanner::new("\"\\\"hello\\\"\\n\"".into());
    assert_eq!(s.scan_tokens().unwrap()[0].lexeme, "\"hello\"\n");
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

    let ports = config["server"]["port"].keys_as::<i32>().unwrap();
    assert_eq!(ports, vec![80, 443]);
}


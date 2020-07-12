use nccl::*;

#[test]
fn pair_keys() {
    let mut p = Pair::new("top");
    p.add("numbers");
    p["numbers"].add("1");
    p["numbers"].add("2");
    p["numbers"].add("3");
    p["numbers"].add("4");
    p["numbers"].add("5");
    assert_eq!(
        p["numbers"].keys_as::<String>().unwrap(),
        vec!["1", "2", "3", "4", "5"]
    );
}

#[test]
fn pair_value_parse() {
    let mut p = Pair::new("top");
    p.add("bools");
    p["bools"].add(true);
    assert!(p["bools"].value_as::<bool>().unwrap());
}

#[test]
fn error_key_not_found() {
    let mut p = Pair::new("jjj");
    assert!(p.get("jwiofiojwaef jio").is_err());
}

#[test]
fn scan_file() {
    assert!(parse_file("examples/config.nccl").is_ok());
}

#[test]
fn dos_unix_lines() {
    assert!(parse_file("examples/config.nccl").is_ok());
    assert!(parse_file("examples/config_dos.nccl").is_ok());
}

#[test]
fn add_pair() {
    // create a new pair
    let mut p1 = Pair::new("happy birthday");

    p1.add("Bobby");
    p1["Bobby"].add("Today!");

    // we think Ron's birthday is the 3rd...
    p1.add("Ron");
    p1["Ron"].add("March 3rd");

    // whoops, we were wrong
    let mut p2 = Pair::new("Ron");
    p2.add("March 2nd");

    // there you go Ron, happy belated birthday
    p1.add_pair(p2);
}

#[test]
fn traverse_path() {
    let mut p = Pair::new("top");
    p.add_slice(&["a".into(), "b".into(), "c".into()]);
    p.traverse_path(&["a".into(), "b".into()]).add("happy");
    assert_eq!(
        p.traverse_path(&["a".into(), "b".into(), "happy".into()]),
        &mut Pair::new("happy")
    );
}

#[test]
fn add_slice() {
    let mut config = Pair::new("top_level");
    config.add("server");
    config["server"].add("domain");
    config["server"].add("port");
    config["server"].add("root");
    config["server"]["domain"].add("example.com");
    config["server"]["domain"].add("www.example.com");
    config["server"]["port"].add("80");
    config["server"]["port"].add("443");
    config["server"]["root"].add("/var/www/html");

    config.add_slice(&["server".into(), "port".into(), "22".into()]);
    assert_eq!(
        config["server"]["port"].keys_as::<String>().unwrap(),
        vec!["80", "443", "22"]
    );
}

#[test]
fn add_vec() {
    let mut p = Pair::new("__top_level__");
    p.add("a");
    p.add_slice(&["a".into(), "hello".into(), "world".into()]);
    p.add_slice(&["a".into(), "hello".into(), "world".into()]);
    assert_eq!(p["a"]["hello"].keys_as::<String>().unwrap().len(), 2);
}

#[test]
fn long() {
    let oh_dear = parse_file("examples/long.nccl").unwrap();
    oh_dear.pretty_print();
}

#[test]
fn inherit2() {
    let schemas = parse_file("examples/inherit.nccl").unwrap();
    let user = parse_file_with("examples/inherit2.nccl", schemas).unwrap();
    assert_eq!(
        user["sandwich"]["meat"].keys_as::<String>().unwrap().len(),
        3
    );
    assert_eq!(user["hello"]["world"].keys_as::<String>().unwrap().len(), 3);
}

#[test]
fn tabs() {
    assert!(parse_file("examples/tabs.nccl").is_err());
}

#[test]
fn spaces() {
    assert!(parse_file("examples/spaces").is_err());
}

#[test]
fn comments() {
    let z = parse_file("examples/comments.nccl").unwrap();
    z.pretty_print();
    assert_eq!("bone hurting juice", z["oof ouch owie"].value().unwrap());
    assert_eq!("another one!", z["no quotes as well"].value().unwrap());
    assert_eq!("perhaps?", z["at the end"].value().unwrap());
}

#[test]
fn indent() {
    assert!(parse_file("examples/indent.nccl").is_ok());
}

#[test]
fn escapes() {
    let p = parse_file("examples/escapes.nccl").unwrap();
    assert_eq!(
        p["hello"].value_as::<String>().unwrap(),
        "people of the earth\nhow's it doing?\""
    );
}

#[test]
fn has_path() {
    let p = parse_file("examples/inherit2.nccl").unwrap();
    assert!(p.has_path(vec_into!["hello", "world", "alaska"]));
}

#[test]
fn readme() {
    let config = parse_file("examples/config.nccl").unwrap();
    let ports = config["server"]["port"].keys_as::<i64>().unwrap();
    assert_eq!(ports, vec![80, 443]);
}

#[test]
fn value() {
    let config = parse_file("examples/long.nccl").unwrap();
    assert_eq!(config["bool too"].value().unwrap(), "false");
}

#[test]
fn duplicates() {
    let config = parse_file("examples/duplicates.nccl").unwrap();
    assert_eq!(
        config["something"].keys_as::<String>().unwrap(),
        vec!["with", "duplicates", "duplicates"]
    );
}

#[test]
fn duplicates2() {
    let config = parse_file("examples/duplicates.nccl").unwrap();
    let config2 = parse_file_with("examples/duplicates2.nccl", config).unwrap();
    assert_eq!(3, config2["something"].keys_as::<String>().unwrap().len());
}

#[test]
fn duplicates3() {
    let config = parse_file("examples/duplicates2.nccl").unwrap();
    let config2 = parse_file_with("examples/duplicates.nccl", config).unwrap();
    assert_eq!(3, config2["something"].keys_as::<String>().unwrap().len());
}

#[test]
fn comments2() {
    let config = parse_string(
        r#"x
# comment
    something
    # comment again
        bingo

does this work?
    who knows
# I sure don't
    is this a child?
"#,
    )
    .unwrap();

    assert!(config["x"]["something"].value().unwrap() == "bingo");
    config["does this work?"].get_ref("who knows").unwrap();
    config["does this work?"]
        .get_ref("is this a child?")
        .unwrap();
}

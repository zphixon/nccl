
//! Nccl is an easy way to add minimal configuration to your crate without
//! having to deal with complicated interfaces, obnoxious syntax, or outdated
//! languages. Nccl makes it easy for a user to pick up your configuration.
//! It's as easy as five cents.
//!
//! Nccl was motivated by the fact that other configuration languages are too
//! complicated for end-users. Strict enforcement of data types is a hassle for
//! people who just want stuff to do things. In nccl's case, not having data
//! types is motivated by the fact that a lot of configuration isn't based on
//! numbers, dates, or booleans, but strings. In the case where a more
//! complicated format is required, it's as simple as a call to `.value_as()` or
//! `.keys_as()`.

mod error;
mod pair;
mod parser;
mod token;
mod scanner;
mod value;

pub use error::*;
pub use pair::*;
pub use value::*;

use parser::*;
use scanner::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

/// Parses a file using the given filename.
///
/// Examples:
///
/// ```
/// let config = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = config["server"]["port"].keys_as::<i64>();
/// assert_eq!(ports, vec![80, 443]);
/// ```
pub fn parse_file(filename: &str) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new(Scanner::new(data).scan_tokens()?).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}

/// Parses a file, merging the results with the supplied pair. Allows for a
/// kind of inheritance of configuration.
///
/// Examples:
///
/// ```
/// let schemas = nccl::parse_file("examples/inherit.nccl").unwrap();
/// let user = nccl::parse_file_with("examples/inherit2.nccl", schemas).unwrap();
/// assert_eq!(user["sandwich"]["meat"].keys().len(), 3);
/// assert_eq!(user["hello"]["world"].keys().len(), 3);
/// ```
pub fn parse_file_with(filename: &str, pair: Pair) -> Result<Pair, Vec<Box<Error>>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Parser::new_with(Scanner::new(data).scan_tokens()?, pair).parse()
    } else {
        Err(vec![Box::new(NcclError::new(ErrorKind::FileError, "Could not find file.", 0))])
    }
}

/// Parses raw string data.
///
/// Examples:
///
/// ```
/// let raw = nccl::parse_string("hello\n\tworld!").unwrap();
/// assert_eq!(raw["hello"].value_as::<String>().unwrap(), "world!");
/// ```
pub fn parse_string(data: &str) -> Result<Pair, Vec<Box<Error>>> {
    Parser::new(Scanner::new(data.to_owned()).scan_tokens()?).parse()
}

#[cfg(test)]
mod tests {
    #[test]
    fn pair_keys() {
        let mut p = ::Pair::new("top");
        p.add("numbers");
        p["numbers"].add("1");
        p["numbers"].add("2");
        p["numbers"].add("3");
        p["numbers"].add("4");
        p["numbers"].add("5");
        assert_eq!(p["numbers"].keys_as::<String>(), vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn pair_value_parse() {
        let mut p = ::Pair::new("top");
        p.add("bools");
        p["bools"].add(true);
        assert!(p["bools"].value_as::<bool>().unwrap());
    }

    #[test]
    fn scanner_literal() {
        let mut s = ::Scanner::new("\"this: is neato\": burrito 64\n    yes.".into());
        s.scan_tokens().unwrap();
    }

    #[test]
    fn error_key_not_found() {
        let mut p = ::Pair::new("jjj");
        assert!(p.get("jwiofiojwaef jio").is_err());
    }

    #[test]
    fn scan_file() {
        assert!(::parse_file("examples/config.nccl").is_ok());
    }

    #[test]
    fn dos_unix_lines() {
        assert!(::parse_file("examples/config.nccl").is_ok());
        assert!(::parse_file("examples/config_dos.nccl").is_ok());
    }

    #[test]
    fn string_escape() {
        let mut s = ::Scanner::new("\"\\\"hello\\\"\\n\"".into());
        assert_eq!(s.scan_tokens().unwrap()[0].lexeme, "\"hello\"\n");
    }

    #[test]
    fn add_pair() {
        // create a new pair
         let mut p1 = ::Pair::new("happy birthday");

         p1.add("Bobby");
         p1["Bobby"].add("Today!");

         // we think Ron's birthday is the 3rd...
         p1.add("Ron");
         p1["Ron"].add("March 3rd");

         // whoops, we were wrong
         let mut p2 = ::Pair::new("Ron");
         p2.add("March 2nd");

         // there you go Ron, happy belated birthday
         p1.add_pair(p2);
    }

    #[test]
    fn traverse_path() {
        let mut p = ::Pair::new("top");
        p.add_slice(&["a".into(), "b".into(), "c".into()]);
        p.traverse_path(&["a".into(), "b".into()]).add("happy");
        assert_eq!(p.traverse_path(&["a".into(), "b".into(), "happy".into()]), &mut ::Pair::new("happy"));
    }

    #[test]
    fn add_slice() {
        let mut config = ::Pair::new("top_level");
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
        assert_eq!(config["server"]["port"].keys_as::<String>(), vec!["80", "443", "22"]);
    }

    #[test]
    fn multiple_errors() {
        let mut s = ::Scanner::new("hey: momma\n   test\n\tjeii\n    oh no!\n".into());
        assert!(s.scan_tokens().is_err());
    }

    #[test]
    fn add_vec() {
        let mut p = ::Pair::new("__top_level__");
        p.add("a");
        p.add_slice(&["a".into(), "hello".into(), "world".into()]);
        p.add_slice(&["a".into(), "hello".into(), "world".into()]);
        assert_eq!(p["a"]["hello"].keys_as::<String>().len(), 1);
    }

    #[test]
    fn long() {
        let oh_dear = ::parse_file("examples/long.nccl").unwrap();
        oh_dear.pretty_print();
    }

    #[test]
    fn inherit2() {
        let schemas = ::parse_file("examples/inherit.nccl").unwrap();
        let user = ::parse_file_with("examples/inherit2.nccl", schemas).unwrap();
        assert_eq!(user["sandwich"]["meat"].keys_as::<String>().len(), 3);
        assert_eq!(user["hello"]["world"].keys_as::<String>().len(), 3);
    }

    #[test]
    fn tabs() {
        assert!(::parse_file("examples/tabs.nccl").is_err());
    }

    #[test]
    fn spaces() {
        assert!(::parse_file("examples/spaces").is_err());
    }

    #[test]
    fn comments() {
        assert!(::parse_file("examples/comments.nccl").is_ok());
    }

    #[test]
    fn indent() {
        assert!(::parse_file("examples/indent.nccl").is_ok());
    }

    #[test]
    fn escapes() {
        let p = ::parse_file("examples/escapes.nccl").unwrap();
        assert_eq!(p["hello"].value_as::<String>().unwrap(), "people of the earth\nhow's it doing?\"");
    }

    #[test]
    fn readme() {
        let config = ::parse_file("examples/config.nccl").unwrap();
        let ports = config["server"]["port"].keys_as::<i64>();
        assert_eq!(ports, vec![80, 443]);
    }
}


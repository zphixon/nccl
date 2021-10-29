//! Nccl is an easy way to add minimal configuration to your crate without
//! having to deal with complicated interfaces, obnoxious syntax, or outdated
//! languages. Nccl makes it easy for a user to pick up your configuration.
//! It's as easy as five cents.
//!
//! Nccl was motivated by the fact that other configuration languages are too
//! complicated for end-users. Strict enforcement of data types is a hassle for
//! people who just want stuff to do things. In nccl's case, simply inferring
//! the data type is a great middle ground between user and developer comfort.

mod error;
mod macros;
mod pair;
mod parser;
mod scanner;
mod token;
mod value;

pub use error::*;
pub use macros::*;
pub use pair::*;
pub use value::*;

use parser::*;
use scanner::*;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_config(content: &str) -> Result<Config, NcclError> {
    let mut scanner = scanner::Scanner2::new(content);
    parse(&mut scanner)
}

pub fn parse_config_with<'orig, 'new>(
    config: &Config<'orig, 'orig>,
    content: &'new str,
) -> Result<Config<'new, 'new>, NcclError>
where
    'orig: 'new,
{
    let mut scanner = scanner::Scanner2::new(content);
    parse_with(&mut scanner, config)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn index() {
        let content = read_to_string("examples/config.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(config["server"]["root"].value(), Some("/var/www/html"));
    }

    #[test]
    fn values() {
        let content = read_to_string("examples/config.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(
            vec![80, 443],
            config["server"]["port"]
                .values()
                .map(|port| port.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
                .unwrap()
        );
    }

    #[test]
    fn value() {
        let content = read_to_string("examples/long.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(config["bool too"].value().unwrap(), "false");
    }

    #[test]
    fn duplicates() {
        let content = read_to_string("examples/duplicates.nccl").unwrap();
        let config = parse_config(&content).unwrap();
        assert_eq!(
            config["something"].values().collect::<Vec<_>>(),
            vec!["with", "duplicates"]
        );
    }

    #[test]
    fn duplicates2() {
        let content1 = read_to_string("examples/duplicates.nccl").unwrap();
        let config1 = parse_config(&content1).unwrap();

        let content2 = read_to_string("examples/duplicates2.nccl").unwrap();
        let config2 = parse_config_with(&config1, &content2).unwrap();

        assert_eq!(2, config2["something"].values().collect::<Vec<_>>().len());
    }

    #[test]
    fn inherit() {
        let sc = read_to_string("examples/inherit.nccl").unwrap();
        let uc = read_to_string("examples/inherit2.nccl").unwrap();

        let schema = parse_config(&sc).unwrap();
        let user = parse_config_with(&schema, &uc).unwrap();

        assert_eq!(3, user["hello"]["world"].values().collect::<Vec<_>>().len());
        assert_eq!(
            3,
            user["sandwich"]["meat"].values().collect::<Vec<_>>().len()
        );
    }

    #[test]
    fn comments() {
        let config = r#"x
# comment
    something
    # comment again
        bingo

does this work?
    who knows
# I sure don't
    is this a child?
"#;
        let config = parse_config(config).unwrap();

        assert_eq!(config["x"]["something"].value().unwrap(), "bingo");
        assert!(config["does this work?"].has_value("who knows"));
        assert!(config["does this work?"].has_value("is this a child?"));
    }

    #[test]
    fn all_of_em() {
        let source = read_to_string("examples/all-of-em.nccl").unwrap();
        let mut scanner = Scanner2::new(&source);
        let config = parse(&mut scanner).unwrap();
        assert_eq!(
            Ok(vec![
                String::from("i # j"),
                String::from("k"),
                String::from("m")
            ]),
            config["h"]
                .children()
                .map(|config| config.parse_quoted())
                .collect::<Result<Vec<_>, _>>()
        );
    }

    #[test]
    fn escapes() {
        let config = read_to_string("examples/escapes.nccl").unwrap();
        let config = parse_config(&config).unwrap();
        assert_eq!(
            config["hello"].child().unwrap().parse_quoted().unwrap(),
            "people of the earth\nhow's it doing?\""
        );
    }
}

/// Parses a file using the given filename.
///
/// Examples:
///
/// ```
/// let config = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = config["server"]["port"].keys_as::<i64>().unwrap();
/// assert_eq!(ports, vec![80, 443]);
/// ```
pub fn parse_file(filename: &str) -> Result<Pair, Vec<NcclError>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|_| vec![NcclError::new(ErrorKind::Io, "IO error", 0)])?;
        Parser::new(Scanner::new(data).scan_tokens()?).parse()
    } else {
        Err(vec![NcclError::new(
            ErrorKind::File,
            "Could not find file.",
            0,
        )])
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
/// assert_eq!(user["sandwich"]["meat"].keys_as::<String>().unwrap().len(), 3);
/// assert_eq!(user["hello"]["world"].keys_as::<String>().unwrap().len(), 3);
/// ```
pub fn parse_file_with(filename: &str, pair: Pair) -> Result<Pair, Vec<NcclError>> {
    if let Ok(mut file) = File::open(Path::new(filename)) {
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|_| vec![NcclError::new(ErrorKind::Io, "IO error", 0)])?;
        Parser::new_with(Scanner::new(data).scan_tokens()?, pair).parse()
    } else {
        Err(vec![NcclError::new(
            ErrorKind::File,
            "Could not find file.",
            0,
        )])
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
pub fn parse_string(data: &str) -> Result<Pair, Vec<NcclError>> {
    Parser::new(Scanner::new(data.to_owned()).scan_tokens()?).parse()
}

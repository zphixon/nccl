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

pub fn parse_config<'a>(content: &'a str) -> Result<Config<'a, 'a>, NcclError> {
    let tokens = scan(content)?;
    parse(&tokens)
}

pub fn parse_config_with<'orig, 'new>(
    config: &Config<'orig, 'orig>,
    content: &'new str,
) -> Result<Config<'new, 'new>, NcclError>
where
    'orig: 'new,
{
    let tokens = scan(content)?;
    parse_with(&tokens, config)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pconfig() {
        let s = std::fs::read_to_string("examples/config.nccl").unwrap();
        let config = parse_config(&s).unwrap();
        panic!("{:#?}", config);
    }

    #[test]
    fn pconfig_with() {
        let s1 = std::fs::read_to_string("examples/inherit.nccl").unwrap();
        let s2 = std::fs::read_to_string("examples/inherit2.nccl").unwrap();
        let config = parse_config(&s1).unwrap();
        let config2 = parse_config_with(&config, &s2).unwrap();
        panic!("{:#?}", config2);
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

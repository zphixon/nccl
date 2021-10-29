//! Nccl is an easy way to add minimal configuration to your crate without
//! having to deal with complicated interfaces, obnoxious syntax, or outdated
//! languages. Nccl makes it easy for a user to pick up your configuration.
//! It's as easy as five cents.
//!
//! Nccl was motivated by the fact that other configuration languages are too
//! complicated for both end-users and developers. Everyone loves types, but
//! in a configuration language, there's too much room for interpretation ( ).
//! There are no types here.
//!
//! This is a nice approach for smaller, non-semantic configurations; you
//! shouldn't be accidentally implementing a DSL with nccl, and if you are, it
//! should feel painful.

pub mod config;
pub mod error;
mod parser;
mod scanner;
mod token;

pub use config::Config;
pub use error::NcclError;

/// Parse a nccl configuration
///
/// e.g.
/// ```
/// # use nccl::*;
/// // config.nccl:
/// // server
/// //     domain
/// //         example.com
/// //         www.example.com
/// //     port
/// //         80
/// //         443
/// //     root
/// //         /var/www/html
///
/// // read the config file
/// let content = std::fs::read_to_string("examples/config.nccl").unwrap();
///
/// // parse it
/// let config = parse_config(&content).unwrap();
///
/// // look ma, no types!
/// assert_eq!(config["server"]["root"].value(), Some("/var/www/html"));
/// ```
pub fn parse_config(content: &str) -> Result<Config, NcclError> {
    let mut scanner = scanner::Scanner::new(content);
    parser::parse(&mut scanner)
}

/// Parse a new nccl configuration on top of another
///
/// e.g.
/// ```
/// # use nccl::*;
/// // user.nccl:
/// // beans
/// //    four
///
/// // default.nccl:
/// // frog
/// //     yes
/// // beans
/// //     none
///
/// // first get the user config
/// let user = std::fs::read_to_string("examples/user.nccl").unwrap();
/// let user_config = parse_config(&user).unwrap();
///
/// // then merge the default config on top of the user config
/// let default = std::fs::read_to_string("examples/default.nccl").unwrap();
/// let combined_config = parse_config_with(&user_config, &default).unwrap();
///
/// // with value(), the first key inserted is returned. since we read the user
/// // config first, the user-supplied value is first, overriding the default.
/// assert_eq!(combined_config["beans"].value(), Some("four"));
/// // "beans" now has two values
/// assert_eq!(combined_config["beans"].values().count(), 2);
///
/// // and the unmodified key remains
/// assert_eq!(combined_config["frog"].value(), Some("yes"));
/// ```
pub fn parse_config_with<'orig, 'new>(
    config: &Config<'orig, 'orig>,
    content: &'new str,
) -> Result<Config<'new, 'new>, NcclError>
where
    'orig: 'new,
{
    let mut scanner = scanner::Scanner::new(content);
    parser::parse_with(&mut scanner, config)
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
        let mut scanner = scanner::Scanner::new(&source);
        let config = parser::parse(&mut scanner).unwrap();
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

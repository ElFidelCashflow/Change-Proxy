use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, str::Split};

use tracing::{debug, trace};

use super::args::Commands;
// use super::write_file;

// const APT_PROXY_PATH: &str = "/etc/apt/apt.conf.d/00proxy";
const APT_PROXY_PATH: &str = "/home/thibault/Documents/perso/rust/change-proxy/target/apt.conf";

#[derive(Debug, Clone, PartialEq, Eq)]
enum ContentType {
    Scope(Scope),
    String(String),
    Bool(bool),
    Integer(i64),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    name: String,
    content: HashMap<String, ContentType>,
}

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let apt_proxy_conf_path: PathBuf = PathBuf::from(APT_PROXY_PATH);
    let content: String = fs::read_to_string(&apt_proxy_conf_path)?;
    trace!("Content of {APT_PROXY_PATH} :\n{}", &content);
    let apt_configuration = parse_apt_config(content);
    dbg!(&apt_configuration);
    Ok(())
}

fn parse_apt_config(content: String) -> HashMap<String, Scope> {
    let mut apt_configuration = HashMap::new();
    let content_splitted = content.split(';');
    debug!("Content splitted: {:#?}", &content_splitted);
    content_splitted.for_each(|configuration_line| {
        let configuration_line_splitted: Split<&str>;
        if configuration_line.contains("::") {
            configuration_line_splitted = configuration_line.split("::");
        } else {
            configuration_line_splitted = configuration_line.split("{");
        }
        let configuration = configuration_line_splitted.last().unwrap();
        dbg!(configuration);
    });
    /*
    apt_configuration.insert(
        String::from("APT"),
        Scope {
            name: String::from("ATP"),
            content: HashMap::new(),
        },
    );
    */
    apt_configuration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bracket() {
        let content = String::from(
            r#"APT {
                Get {
                    Assume-Yes "true";
                };
            };
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal = Scope {
            name: String::from("APT"),
            content: HashMap::from([(
                String::from("Get"),
                ContentType::Scope(Scope {
                    name: String::from("Get"),
                    content: HashMap::from([(String::from("Assume-Yes"), ContentType::Bool(true))]),
                }),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal)
        );
    }

    #[test]
    fn complete_brackets() {
        let content = String::from(
            r#"APT {
                Get {
                    Assume-Yes "true";
                    Fix-Broken "true";
                };
            };
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal = Scope {
            name: String::from("APT"),
            content: HashMap::from([(
                String::from("Get"),
                ContentType::Scope(Scope {
                    name: String::from("Get"),
                    content: HashMap::from([
                        (String::from("Assume-Yes"), ContentType::Bool(true)),
                        (String::from("Fix-Broken"), ContentType::Bool(true)),
                    ]),
                }),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal)
        );
    }

    #[test]
    fn simple_monoline() {
        let content = String::from(
            r#"
            APT::Get::Assume-Yes "true";
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal = Scope {
            name: String::from("APT"),
            content: HashMap::from([(
                String::from("Get"),
                ContentType::Scope(Scope {
                    name: String::from("Get"),
                    content: HashMap::from([(String::from("Assume-Yes"), ContentType::Bool(true))]),
                }),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal)
        );
    }

    #[test]
    fn complete_monolines() {
        let content = String::from(
            r#"
            APT::Get::Assume-Yes "true";
            APT::Get::Fix-Broken "true";
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal = Scope {
            name: String::from("APT"),
            content: HashMap::from([(
                String::from("Get"),
                ContentType::Scope(Scope {
                    name: String::from("Get"),
                    content: HashMap::from([
                        (String::from("Assume-Yes"), ContentType::Bool(true)),
                        (String::from("Fix-Broken"), ContentType::Bool(true)),
                    ]),
                }),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal)
        );
    }

    #[test]
    fn complete_hybride() {
        let content = String::from(
            r#"
                APT::Get::{Assume-Yes "true"; Fix-Broken "true"};
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal = Scope {
            name: String::from("APT"),
            content: HashMap::from([(
                String::from("Get"),
                ContentType::Scope(Scope {
                    name: String::from("Get"),
                    content: HashMap::from([
                        (String::from("Assume-Yes"), ContentType::Bool(true)),
                        (String::from("Fix-Broken"), ContentType::Bool(true)),
                    ]),
                }),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal)
        );
    }
}

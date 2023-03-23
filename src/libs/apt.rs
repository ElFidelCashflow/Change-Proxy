use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use tracing::trace;

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
    List(Vec<String>),
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

/*fn parse_apt_config(content: String) -> HashMap<String, Scope> {
    let mut parent_configuration = HashMap::new();
    let content_splitted = content.split(';');
    debug!("Content splitted: {:#?}", &content_splitted);
    content_splitted.for_each(|configuration_line| {
        // Line cleaning
        let configuration_line = configuration_line.replace('\n', "");
        let mut configuration_line_splitted: Split<&str>;
        // Determine how to split the configuration
        if configuration_line.contains("::") {
            configuration_line_splitted = configuration_line.split("::");
        } else {
            configuration_line_splitted = configuration_line.split("{");
        }
        let parent_configuration_from_str = configuration_line_splitted.next().unwrap();
        // Parser les 1ers arguments et limite faire une fonction récursive jusqu'à avoir la derniere configuraiton possible

        //     Sinon:
        // let start_bytes = s.find('{').unwrap_or(0); //index where "pattern" starts
        //      // or beginning of line if
        //      // "pattern" not found
        // let end_bytes = s.rfind('}').unwrap_or(s.len()); //index where "<" is found
        //     // or end of line

        // let result = &s[start_bytes+1..end_bytes];

        //     Autre solution c'est de compter les {} correspondants

        // Get last info of lines containing the confuguration name
        let configuration = configuration_line_splitted.last().unwrap();
        // Controle if the line is not empty
        if !configuration.replace(' ', "").is_empty() {
            let conf_splitted: Vec<&str> = configuration.split(' ').collect();
            parent_configuration.insert(
                String::from(conf_splitted[0]),
                ContentType::Bool(conf_splitted[1].replace('"', "").parse::<bool>().unwrap()),
            );
        }
    });
    // parent_configuration
    HashMap::from([(
        String::from("APT"),
        Scope {
            name: String::from("APT"),
            content: parent_configuration,
        },
    )])
}*/

fn get_matching_pair(content: String) -> Scope {
    let mut starting_tags: Vec<usize> = Vec::new();
    let mut matching_pairs: Vec<(usize, usize)> = Vec::new();
    let mut root_matching_pair = false;
    println!("{}", &content);
    content.chars().enumerate().for_each(|(char_index, char)| {
        if !root_matching_pair {
            if char == '{' {
                starting_tags.push(char_index);
            } else if char == '}' {
                matching_pairs.push((starting_tags.pop().unwrap(), char_index));
                if starting_tags.len() == 0 {
                    root_matching_pair = true
                }
            }
        }
    });
    Scope {
        name: String::from(content[..matching_pairs.last().unwrap().0].trim()),
        content: HashMap::from([(
            String::from("Test"),
            ContentType::String(String::from(
                &content[matching_pairs.last().unwrap().0 + 1..matching_pairs.last().unwrap().1],
            )),
        )]),
    }
}

fn parse_apt_config(content: String) -> HashMap<String, Scope> {
    let mut parent_configuration = HashMap::new();
    let root_configuration = get_matching_pair(content);
    dbg!(&root_configuration);
    let root_configuration_name = &root_configuration.name;
    parent_configuration.insert(String::from(root_configuration_name), root_configuration);
    parent_configuration
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
                APT {
                    Get {
                        Assume-Yes "true";
                    };
                    Cache
                    {
                        AllNames "true";
                    };
                };
                Acquire::Languages { "environment"; "fr"; "en"; "none"; "de"; };
            "#,
        );
        let dict_scopes_parsed = parse_apt_config(content);
        let scope_goal_apt = Scope {
            name: String::from("APT"),
            content: HashMap::from([
                (
                    String::from("Get"),
                    ContentType::Scope(Scope {
                        name: String::from("Get"),
                        content: HashMap::from([(
                            String::from("Assume-Yes"),
                            ContentType::Bool(true),
                        )]),
                    }),
                ),
                (
                    String::from("Cache"),
                    ContentType::Scope(Scope {
                        name: String::from("Cache"),
                        content: HashMap::from([(
                            String::from("AllNames"),
                            ContentType::Bool(true),
                        )]),
                    }),
                ),
            ]),
        };
        let scope_goal_acquire = Scope {
            name: String::from("Acquire"),
            content: HashMap::from([(
                String::from("Languages"),
                ContentType::List(Vec::from([
                    String::from("environment"),
                    String::from("fr"),
                    String::from("en"),
                    String::from("none"),
                    String::from("de"),
                ])),
            )]),
        };
        assert_eq!(
            dict_scopes_parsed.get(&String::from("APT")),
            Some(&scope_goal_apt)
        );
        assert_eq!(
            dict_scopes_parsed.get(&String::from("Acquire")),
            Some(&scope_goal_acquire)
        )
    }
}

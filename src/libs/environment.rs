use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::slice::Iter;

extern crate tracing;
use tracing::{debug, info, trace, warn};

use super::args::Commands;
use super::write_file;
use super::ProxyType;

const ENV_PATH: &str = "/etc/environment";

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match *self {
            Self::Http => "http_proxy",
            Self::Https => "https_proxy",
            Self::Ftp => "ftp_proxy",
            Self::NoProxy => "no_proxy",
        };
        write!(f, "{format}")
    }
}

impl ProxyType {
    pub fn iterator() -> Iter<'static, ProxyType> {
        static PROXY_TYPES: [ProxyType; 4] = [
            ProxyType::Http,
            ProxyType::Https,
            ProxyType::Ftp,
            ProxyType::NoProxy,
        ];
        PROXY_TYPES.iter()
    }
}

fn get_content_as_vec() -> Result<Vec<String>, Box<dyn Error>> {
    let general_env_path: PathBuf = PathBuf::from(ENV_PATH);
    let content: String = fs::read_to_string(general_env_path)?;
    trace!("Content of {ENV_PATH} :\n{}", &content);
    Ok(content
        .lines()
        .map(|line| line.into())
        .collect::<Vec<String>>())
}

pub fn get_configuration() -> Option<String> {
    let content_as_vec = get_content_as_vec().ok()?;
    if let Some(url_proxy) = content_as_vec
        .iter()
        .find(|line| line.contains(format!("{}", ProxyType::Http).as_str()))
    {
        let url_proxy_stripped = url_proxy
            .split('=')
            .last()
            .unwrap()
            .to_string()
            .replace('\"', "");
        Some(url_proxy_stripped)
    } else {
        None
    }
}

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let mut content_as_vec = match get_content_as_vec() {
        Ok(content_as_vec) => content_as_vec,
        Err(_) => {
            warn!("{ENV_PATH} does not exist. Creating file");
            Vec::new()
        }
    };
    let general_env_path: PathBuf = PathBuf::from(ENV_PATH);
    match subcommand {
        Commands::Add { proxy_url } => {
            let proxy_url = format!("{}{}{}", '\"', proxy_url, '\"');
            ProxyType::iterator().for_each(|proxy_type| {
                for case in [
                    format!("{proxy_type}"),
                    format!("{proxy_type}").to_uppercase(),
                ] {
                    match proxy_type {
                        ProxyType::NoProxy => debug!("{case} : Not yet implemented"),
                        _ => {
                            let new_proxy_line = format!("{case}={proxy_url}");
                            if let Some((index, _)) = content_as_vec
                                .iter()
                                .enumerate()
                                .find(|line| line.1.contains(&case.to_string()))
                            {
                                debug!("Replacing existing configuration for {case}");
                                content_as_vec.remove(index);
                                content_as_vec.insert(index, new_proxy_line);
                            } else {
                                debug!("Adding new configuration");
                                content_as_vec.push(new_proxy_line);
                            }
                        }
                    };
                }
            });
        }
        Commands::Show => {
            if let Some(url_proxy) = get_configuration() {
                info!("Proxy used : {}", url_proxy);
            } else {
                info!("No proxy used");
            }
            return Ok(());
        }
        Commands::Remove => {
            ProxyType::iterator().for_each(|proxy_type| {
                debug!("Removing configuration for {proxy_type} in /etc/environment");
                content_as_vec.retain(|line| {
                    !line
                        .to_lowercase()
                        .contains(format!("{proxy_type}").as_str())
                });
            });
        }
    }
    let content_rebuild: String = content_as_vec.join("\n");
    trace!("Content rebuild : \n{}", &content_rebuild);
    write_file(&general_env_path, &content_rebuild)?;
    info!("Proxy configuration done for /etc/environment");
    Ok(())
}

use self::ProxyType::*;
use super::write_file;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::slice::Iter;

use tracing::{debug, info, trace};

use super::args::Commands;

#[derive(PartialEq, Eq)]
enum ProxyType {
    Http,
    Https,
    Ftp,
    NoProxy,
}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match *self {
            Self::Http => "HTTP_PROXY",
            Self::Https => "HTTPS_PROXY",
            Self::Ftp => "FTP_PROXY",
            Self::NoProxy => "NO_PROXY",
        };
        write!(f, "{}", format)
    }
}

impl ProxyType {
    pub fn iterator() -> Iter<'static, ProxyType> {
        static PROXY_TYPES: [ProxyType; 4] = [Http, Https, Ftp, NoProxy];
        PROXY_TYPES.iter()
    }
}

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let general_env_path: PathBuf = PathBuf::from("/etc/environment");
    let content: String = fs::read_to_string(&general_env_path)?;
    trace!("Content of /etc/environment :\n{}", &content);
    let mut content_as_vec = content
        .lines()
        .map(|line| line.into())
        .collect::<Vec<String>>();

    match subcommand {
        Commands::Add { proxy_url } => {
            let proxy_url = format!("{}{}{}", '\"', proxy_url, '\"');
            let no_proxy_conf = "\"\"".to_string();
            ProxyType::iterator().for_each(|proxy_type| {
                let proxy_conf = if *proxy_type == NoProxy {
                    &no_proxy_conf
                } else {
                    &proxy_url
                };
                if let Some((index, _)) = content_as_vec
                    .iter()
                    .enumerate()
                    .find(|line| line.1.contains(format!("{proxy_type}").as_str()))
                {
                    debug!("Replacing exisitng configuration of {proxy_type} with {proxy_conf}");
                    trace!("BEFORE :\n{:#?}", content_as_vec);
                    let new_proxy_line = format!("{}={}", proxy_type, proxy_conf);
                    content_as_vec.remove(index);
                    content_as_vec.insert(index, new_proxy_line);
                    trace!("AFTER :\n{:#?}", content_as_vec);
                } else {
                    debug!("Adding new proxy configuration for {proxy_type} with {proxy_conf}");
                    content_as_vec.push(format!("{proxy_type}={proxy_conf}"));
                }
            });
            dbg!(&content_as_vec);
            let content_rebuild: String = content_as_vec.join("\n");
            debug!("Content rebuild : \n{}", &content_rebuild);
            write_file(&general_env_path, &content_rebuild)?
        }
        Commands::Show => {
            if let Some(url_proxy) = content_as_vec
                .iter()
                .find(|line| line.contains(format!("{Http}").as_str()))
            {
                info!("Proxy used : {}", url_proxy.split('=').last().unwrap());
            } else {
                info!("No proxy used");
            }
        }
        Commands::Remove => (),
    }
    todo!();
}

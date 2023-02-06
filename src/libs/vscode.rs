use std::error::Error;
use std::fmt;
use std::path::PathBuf;

extern crate tracing;
use tracing::{debug, error, info};

use super::args::Commands;
use super::get_json_parsed;
use super::write_file;

#[derive(Debug, Clone)]
enum Errors {
    FileNotFound,
    HomeDirMissing,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
impl Error for Errors {}

fn get_configuration_path() -> Result<PathBuf, Box<dyn Error>> {
    debug!("Generating VSCode configuration path");

    let mut possibles_paths: Vec<PathBuf> = vec![];

    if let Some(home_path) = dirs::home_dir() {
        possibles_paths.push(PathBuf::from(format!(
            "{}/.config/Code/User/settings.json",
            home_path.as_path().to_str().unwrap()
        )));
        possibles_paths.push(PathBuf::from(format!(
            "{}/.var/app/com.visualstudio.code/config/Code/User/settings.json",
            home_path.as_path().to_str().unwrap()
        )));
    } else {
        debug!("Home dir not setted");
        return Err(Box::new(Errors::HomeDirMissing));
    }

    if let Some(found_path) = possibles_paths.iter().find(|p| p.exists()) {
        debug!("Configuration path : {}", &found_path.display());
        return Ok(found_path.clone());
    }

    error!("No configurtation file found");
    Err(Box::new(Errors::FileNotFound))
}

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let config_path = get_configuration_path()?;
    let mut content_parsed = get_json_parsed(&config_path)?;
    match subcommand {
        Commands::Add { proxy_url } => {
            debug!("Inserting \"http.proxy\" : {}", &proxy_url);
            content_parsed.insert("http.proxy", proxy_url.clone())?;
        }
        Commands::Remove => {
            debug!("Removing the entry \"http.proxy\"");
            content_parsed.remove("http.proxy");
            debug!("Calling write_file with new content");
        }
        Commands::Show => {
            if content_parsed["http.proxy"] == json::Null {
                info!("No proxy used");
            } else {
                info!("Proxy used : {}", content_parsed["http.proxy"]);
            }
            return Ok(());
        }
    }
    debug!("Calling write_file with new content");
    write_file(&config_path, content_parsed.pretty(4).as_str())?;
    info!("Proxy configuration done for VSCode");
    Ok(())
}

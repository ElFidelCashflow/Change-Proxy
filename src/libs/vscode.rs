use dirs;
use json::JsonValue;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info, trace};

use super::args::Commands;
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

fn get_json_parsed(path: &PathBuf) -> Result<JsonValue, Box<dyn Error>> {
    debug!("Reading content of {}", &path.display());
    let contents = fs::read_to_string(&path)?;
    trace!("Content from file:\n{}", &contents);
    let json_parsed = json::parse(contents.as_str()).unwrap();
    trace!("Json parsed object:\n{}", &json_parsed);
    Ok(json_parsed)
}

pub fn manage_proxy(subcommand: Commands) -> Result<(), Box<dyn Error>> {
    let config_path = get_configuration_path()?;
    let mut content_parsed = get_json_parsed(&config_path)?;
    match subcommand {
        Commands::Add { proxy_url } => {
            debug!("Inserting \"http.proxy\" : {}", &proxy_url);
            content_parsed.insert("http.proxy", proxy_url)?;
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
        }
    }
    debug!("Calling write_file with new content");
    write_file(&config_path, content_parsed.pretty(4).as_str())?;
    info!("Proxy configuration added for VSCode");
    Ok(())
}

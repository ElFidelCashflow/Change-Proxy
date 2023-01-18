use dirs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info, trace};

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

pub fn show_file() -> Result<(), Box<dyn Error>> {
    let vscode_config_file_path = get_configuration_path()?;
    let contents = fs::read_to_string(&vscode_config_file_path)?;
    trace!("Content from file:\n{}", &contents);
    let json_parsed = json::parse(contents.as_str()).unwrap();
    trace!("Json parsed object:\n{}", &json_parsed);
    if json_parsed["http_proxy"] == json::Null {
        info!("No proxy used");
    } else {
        info!("Proxy used : {}", json_parsed["http_proxy"]);
    }
    Ok(())
}

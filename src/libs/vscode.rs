use dirs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tracing::{error, debug, info, trace};


#[derive(Debug, Clone)]
struct FileNotFound;

impl fmt::Display for FileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
impl Error for FileNotFound {}


fn get_configuration_path() -> Result<PathBuf, Box<dyn Error>> {
    debug!("Generating VSCode configuration path");
    let possibles_paths: Vec<PathBuf>  = Vec::from([[
        dirs::home_dir().expect("Home not setted."),
        PathBuf::from(".config/Code/User/settings.json"),
    ]
    .iter()
    .collect(),
    [
        dirs::home_dir().expect("Home not setted."),
        PathBuf::from(".var/app/com.visualstudio.code/config/Code/User/settings.json"),
    ]
    .iter()
    .collect(),
    ]);

    for path in possibles_paths {
        if path.exists() {
            let file_path = path;
            debug!("Configuration path : {}", &file_path.display());
            return Ok(file_path);
        }
    }
    error!("No configurtation file found");
    return Err(FileNotFound.into())
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

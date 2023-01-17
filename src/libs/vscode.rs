use dirs;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, trace};

fn get_configuration_path() -> Result<PathBuf, Box<dyn Error>> {
    debug!("Generating VSCode configuration path");
    let file_path: PathBuf = [
        dirs::home_dir().expect("Home not setted."),
        PathBuf::from(".config/Code/User/settings.json"),
    ]
    .iter()
    .collect();
    debug!("VSCode configuration path : {}", &file_path.display());
    Ok(file_path)
}

pub fn show_file() -> Result<(), Box<dyn Error>> {
    let vscode_config_file_path = get_configuration_path()?;
    let contents = fs::read_to_string(&vscode_config_file_path)?;
    trace!("Content from VSCode configuration path:\n{}", &contents);
    // let json_parsed = json::parse(contents.as_str()).unwrap();
    let json_parsed = json::parse(contents.as_str()).unwrap();
    dbg!(&json_parsed);
    info!("VSCode: Proxy used : {}", json_parsed["http_proxy"]);
    Ok(())
}

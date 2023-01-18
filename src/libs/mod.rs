use std::{error::Error, fs::write, path::Path};
use tracing::{debug, info};
mod vscode;

pub fn add(url: String) -> Result<(), Box<dyn Error>> {
    info!("Adding proxy configurations with the following : {url}");
    vscode::add_proxy(url)?;
    Ok(())
}

pub fn remove() -> Result<(), Box<dyn Error>> {
    info!("Removing existing proxy configurations");
    vscode::remove_proxy()?;
    Ok(())
}

pub fn show() -> Result<(), Box<dyn Error>> {
    info!("Showing proxy used");
    vscode::show_proxy()?;
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing file {}", &path.display());
    write(path, content)?;
    Ok(())
}

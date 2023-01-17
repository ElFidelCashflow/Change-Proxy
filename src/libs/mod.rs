use std::error::Error;
use tracing::{info, warn};
mod vscode;

pub fn add(url: String) -> Result<(), Box<dyn Error>> {
    warn!("Adding proxy {url} : Not yet implemented");
    Ok(())
}

pub fn remove() -> Result<(), Box<dyn Error>> {
    warn!("Removing proxy : Not yet implemented");
    Ok(())
}

pub fn show() -> Result<(), Box<dyn Error>> {
    info!("Showing proxy used");
    vscode::show_file()?;
    Ok(())
}

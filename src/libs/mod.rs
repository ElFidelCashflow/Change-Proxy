use std::error::Error;
use tracing::{info, warn};
mod vscode;

pub fn add(url: String) -> Result<(), Box<dyn Error>> {
    info!("Adding proxy {url}");
    Ok(())
}

pub fn remove() -> Result<(), Box<dyn Error>> {
    warn!("Removing proxy : Not yet implemented");
    Ok(())
}

pub fn show() -> Result<(), Box<dyn Error>> {
    warn!("Proxy used : Not yet implemented");
    vscode::show_file()?;
    Ok(())
}

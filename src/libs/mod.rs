use std::{error::Error, fs::write, path::Path};
use tracing::debug;
pub mod args;
pub mod vscode;

pub fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing file {}", &path.display());
    write(path, content)?;
    Ok(())
}

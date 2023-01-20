use std::{error::Error, fs::write, path::Path};
use tracing::debug;
pub mod args;
pub mod vscode;
use file_owner::PathExt;

pub fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing file {}", &path.display());
    let (owner, group) = &path.owner_group().unwrap();
    debug!("File owned by {}", &owner);
    write(&path, content)?;
    if &path.owner().unwrap() != owner {
        debug!("Resetting the owner to {}", &owner);
        let _ = &path.set_owner_group(*owner, *group)?;
    }
    Ok(())
}

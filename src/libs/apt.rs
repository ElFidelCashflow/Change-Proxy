use super::args::Commands;
use super::write_file;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tracing::trace;

const APT_PROXY_PATH: &str = "/etc/apt/apt.conf.d/00proxy";

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let apt_proxy_conf_path: PathBuf = PathBuf::from(APT_PROXY_PATH);
    let content: String = fs::read_to_string(&apt_proxy_conf_path)?;
    trace!("Content of {APT_PROXY_PATH} :\n{}", &content);
    Ok(())
}

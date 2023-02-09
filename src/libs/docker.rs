extern crate ini;
use ini::Ini;

use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

extern crate tracing;
use tracing::{debug, info, trace, warn};

use super::args::Commands;
use super::environment;
use super::write_file;

const DOCKER_SYSTEMD_PATH: &str = "/etc/systemd/system/docker.service.d/proxy.conf";

pub fn manage_proxy(subcommand: &Commands) -> Result<(), Box<dyn Error>> {
    let mut docker_systemd_conf = match Ini::load_from_file(DOCKER_SYSTEMD_PATH) {
        Ok(ini) => ini,
        Err(_) => Ini::new(),
    };
    let mut buf = Vec::new();
    match subcommand {
        Commands::Add { proxy_url: _ } => {
            info!("Adding proxy configuration for Docker");
            debug!("Making sure that the EnvironmentFile var is set");
            docker_systemd_conf
                .with_section(Some("Service"))
                .set("EnvironmentFile", "-/etc/environment");
            match docker_systemd_conf.write_to(&mut buf) {
                Ok(()) => (),
                Err(err) => warn!("{err}"),
            };
            write_file(
                &PathBuf::from(DOCKER_SYSTEMD_PATH),
                std::str::from_utf8(&buf).unwrap(),
            )?;
        }
        Commands::Remove => {
            info!("Removing proxy configuration for Docker");
        }
        Commands::Show => {
            let service_section = docker_systemd_conf.section(Some("Service")).unwrap();
            trace!("Service section : {:?}", service_section);

            if let Some(url_proxy) = environment::get_configuration() {
                info!(
                    "Proxy used defined in {}: {}",
                    service_section.get("EnvironmentFile").unwrap(),
                    url_proxy
                );
            } else {
                info!("No proxy used");
            }
            return Ok(());
        }
    }
    info!("Calling environment::manage_proxy");
    debug!("with command {subcommand}");
    environment::manage_proxy(subcommand)?;
    debug!("Reload daemon configuration");
    Command::new("systemctl")
        .arg("daemon-reload")
        .spawn()
        .expect("Cannont reload docker service environment");
    debug!("Restarting docker daemon");
    systemctl::restart("docker.service").unwrap();
    info!("Proxy configuration done for Docker");
    Ok(())
}

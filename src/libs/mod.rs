extern crate file_owner;
use file_owner::{Group, Owner, PathExt};

extern crate json;
use json::JsonValue;

use std::{
    error::Error,
    fs::{self, write},
    path::{Path, PathBuf},
};

extern crate tracing;
extern crate tracing_subscriber;
use tracing::{debug, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod apt;
pub mod args;
mod docker;
mod environment;
mod vscode;

#[derive(PartialEq, Eq)]
pub enum ProxyType {
    Http,
    Https,
    Ftp,
    NoProxy,
}

pub fn get_json_parsed(path: &PathBuf) -> Result<JsonValue, Box<dyn Error>> {
    debug!("Reading content of {}", &path.display());
    let contents = fs::read_to_string(path)?;
    trace!("Content from file:\n{}", &contents);
    let json_parsed = json::parse(contents.as_str()).expect("Json not valid");
    trace!("Json parsed object:\n{}", &json_parsed);
    Ok(json_parsed)
}

pub fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing file {}", &path.display());
    let (owner, group) = match path.owner_group() {
        Ok(ownership) => ownership,
        Err(_) => (Owner::from_uid(0), Group::from_gid(0)),
    };
    debug!("File owned by {}", &owner);
    trace!("Writting content: \n{content}");
    write(path, content)?;
    if path.owner().unwrap() != owner {
        debug!("Resetting the owner to {}", &owner);
        path.set_owner_group(owner, group)?;
    }
    Ok(())
}

fn manage_proxy(args: &args::Cli) -> Result<(), Box<dyn Error>> {
    if args.all || args.apt {
        apt::manage_proxy(&args.command)?;
    }
    if args.all || args.docker {
        docker::manage_proxy(&args.command)?;
    }
    if args.all || args.environment {
        environment::manage_proxy(&args.command)?;
    }
    if args.all || args.vscode {
        vscode::manage_proxy(&args.command)?;
    }
    Ok(())
}

pub fn run(args: args::Cli) -> Result<(), Box<dyn Error>> {
    let verbosity: Level = match args.verbosity {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .compact()
        .with_max_level(verbosity)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    debug!("Running as {:?}", sudo::check());
    if sudo::check() != sudo::RunningAs::Root {
        warn!("Upgrading to run as {:?}", sudo::RunningAs::Root);
        sudo::with_env(&["HOME"])?;
    }
    trace!("Env HOME : {}", std::env::var("HOME").unwrap());

    info!("Using {verbosity} level of log");
    manage_proxy(&args)?;
    Ok(())
}

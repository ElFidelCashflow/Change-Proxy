use std::{error::Error, fs::write, path::Path};
use tracing::{debug, info, trace, warn, Level};
pub mod args;
mod environment;
mod vscode;
use file_owner::PathExt;
use tracing_subscriber::FmtSubscriber;

pub fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing file {}", &path.display());
    let (owner, group) = &path.owner_group().unwrap();
    debug!("File owned by {}", &owner);
    trace!("Writting content: \n{content}");
    write(path, content)?;
    if path.owner().unwrap() != *owner {
        debug!("Resetting the owner to {}", &owner);
        path.set_owner_group(*owner, *group)?;
    }
    Ok(())
}

fn manage_proxy(subcommand: &args::Commands) -> Result<(), Box<dyn Error>> {
    vscode::manage_proxy(subcommand)?;
    environment::manage_proxy(subcommand)?;
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
    // write_file(PathBuf::from("/tmp/test").as_path(), "ascoikjnaeofgnéeoijtgnoeéirzht oiéhtoihzaeo tho")?;
    manage_proxy(&args.command)?;
    Ok(())
}

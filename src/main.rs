use std::error::Error;
use std::process;

use clap::Parser;
use tracing::{info, trace, Level, warn};
use tracing_subscriber::FmtSubscriber;
mod libs;

fn main() {
    let args = libs::args::Cli::parse();
    if let Err(_) = run(args) {
        process::exit(1);
    }
}

fn run(args: libs::args::Cli) -> Result<(), Box<dyn Error>> {
    let verbosity: Level;
    match args.verbosity {
        0 => verbosity = Level::INFO,
        1 => verbosity = Level::DEBUG,
        2.. => verbosity = Level::TRACE,
    }
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .compact()
        .with_max_level(verbosity)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    sudo::with_env(&["HOME"])?;
    warn!("Running as {:?}", sudo::check());
    trace!("Env HOME : {}", std::env::var("HOME").unwrap());

    info!("Usinge {verbosity} level of log");
    libs::vscode::manage_proxy(&args.command)?;
    Ok(())
}

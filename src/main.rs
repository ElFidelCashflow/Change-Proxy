use std::error::Error;
use std::process;

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
pub mod args;
pub mod libs;

fn main() {
    let args = args::Cli::parse();

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
        .with_max_level(verbosity)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Usinge {verbosity} level of log");

    if let Err(_) = run(args) {
        process::exit(1);
    }
}

fn run(args: args::Cli) -> Result<(), Box<dyn Error>> {
    match args.command {
        args::Commands::Add { proxy_url } => libs::add(proxy_url),
        args::Commands::Remove => libs::remove(),
        args::Commands::Show => libs::show(),
    }
}

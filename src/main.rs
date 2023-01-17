use clap::Parser;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;
pub mod args;

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

    match args.command {
        args::Commands::Add { proxy_url } => info!("Adding proxy {proxy_url}"),
        args::Commands::Remove => info!("Removing proxy"),
        args::Commands::Show => info!("Proxy used : Not yet implemented"),
    }
    trace!("Test trace");
    debug!("Test degbug");
    warn!("Tests waring");
    error!("Test error");
}

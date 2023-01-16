use clap::{Parser};
pub mod args;

fn main() {
    let args = args::Cli::parse();

    match args.command {
        args::Commands::Add { proxy_url } => println!("Adding proxy {proxy_url}"),
        args::Commands::Remove => println!("Removing proxy"),
        args::Commands::Show => println!("Proxy used : Not yet implemented"),
    }
}

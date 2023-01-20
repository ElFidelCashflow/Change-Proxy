use std::process;

use clap::Parser;
mod libs;

fn main() {
    let args = libs::args::Cli::parse();
    if libs::run(args).is_err() {
        process::exit(1);
    }
}

extern crate clap;
use clap::Parser;

use std::process;

mod libs;

fn main() {
    let args = libs::args::Cli::parse();
    if libs::run(args).is_err() {
        process::exit(1);
    }
}

use clap::{ArgAction::Count, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "change-proxy")]
#[command(about = "Allow to manipulate proxy configuration for many services", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(global = true, short, long, action = Count, aliases = ["verbose", "verbeux"])]
    #[arg(help = "Verbose mode (-vv for more)")]
    pub verbosity: u8,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(about = "Set the proxy given to all services")]
    Add { proxy_url: String },

    #[command(about = "Remove proxy from all services")]
    Remove,

    #[command(about = "Show proxy used")]
    Show,
}

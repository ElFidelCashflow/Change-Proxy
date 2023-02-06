extern crate clap;
use std::fmt::Display;

use clap::{ArgAction::Count, ArgAction::SetTrue, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "change-proxy")]
#[command(
    about = "Allow to manipulate proxy configuration for follinwg services:
    - apt
    - docker
    - /etc/environment
    - vscode"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(global = true, short, long, action = Count, aliases = ["verbose", "verbeux"])]
    #[arg(help = "Verbose mode (-vv for more)\naliases: verbose, verbeux")]
    pub verbosity: u8,

    #[arg(global = true, long, action = SetTrue , help = "Default: [true]")]
    #[arg(default_value = "true",
        default_value_ifs = [
            ("apt", "true", "false"),
            ("docker", "true", "false"),
            ("environment", "true", "false"),
            ("vscode", "true", "false")
            ]
        )]
    pub all: bool,

    #[arg(global = true, long, action = SetTrue)]
    pub apt: bool,

    #[arg(global = true, long, action = SetTrue)]
    pub docker: bool,

    #[arg(global = true, long, action = SetTrue, aliases = ["env", "environement"])]
    pub environment: bool,

    #[arg(global = true, long, action = SetTrue)]
    pub vscode: bool,
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

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = match &self {
            Self::Add { proxy_url } => format!("Add (url: {proxy_url})"),
            Self::Remove => "Remove".to_string(),
            Self::Show => "Show".to_string(),
        };
        write!(f, "{format}")
    }
}

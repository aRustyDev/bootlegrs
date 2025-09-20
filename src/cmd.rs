use crate::{PKG_AUTHORS, PKG_DESC};
use clap::{Parser, Subcommand};
use clap_verbosity_flag;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = PKG_AUTHORS, version, about = PKG_DESC, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "givemeideas")]
    pub recipe: String,

    #[arg(long)]
    pub verbose: bool,

    #[arg(long)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Bootleg>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Bootleg {
    Commands {
        #[arg(long)]
        to: String,
    },
    Completions {
        #[arg(long)]
        shell: String,
    },
    Config {
        #[arg(long)]
        to: String,
    },
    Edit {
        #[arg(long)]
        to: String,
    },
    Health {
        #[arg(long)]
        to: String,
    },
    Info {
        #[arg(long)]
        to: String,
    },
    Init {
        #[arg(long)]
        to: String,
    },
    Install {
        #[arg(long)]
        to: String,
    },
    List {
        #[arg(long)]
        to: String,
    },
    New {
        #[arg(long)]
        to: String,
    },
    Search {
        #[arg(long)]
        to: String,
    },
    Taps {
        #[arg(long)]
        to: String,
    },
    Tui {
        #[arg(long)]
        to: String,
    },
    Uninstall {
        #[arg(long)]
        to: String,
    },
    Update {
        #[arg(long)]
        to: String,
    },
    Upgrade {
        #[arg(long)]
        to: String,
    },
}

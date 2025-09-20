use crate::{PKG_AUTHORS, PKG_DESC};
use clap::{Parser, Subcommand};
use clap_verbosity_flag;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = PKG_AUTHORS, version, about = PKG_DESC, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "givemeideas", help = "repipe hlep info")]
    pub recipe: String,

    // Display any debugging information.
    #[arg(long)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Bootleg>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Bootleg {
    Completions {
        #[arg(
            long,
            help = "Generate Shell Completions [zsh,bash,powershell/pwsh,elvish,nushell/nu]"
        )]
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
    Schema {
        #[arg(long, help = "Generate JSON Schemas", default_value = "all")]
        schema: String,
    },
    Search {
        #[arg(long, help = "Search related actions")]
        to: String,
    },
    Taps {
        #[arg(long, help = "Tap related action")]
        to: String,
    },
    Tui {
        #[arg(long, help = "Start interactive TUI")]
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

use crate::cli::commands::Bootleg;
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

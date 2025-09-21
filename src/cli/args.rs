use crate::cli::commands::Bootleg;
use crate::{PKG_AUTHORS, PKG_DESC};
use clap::Parser;
use clap_verbosity_flag::ErrorLevel;
use clap_verbosity_flag::LogLevel;
use clap_verbosity_flag::Verbosity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// #[derive(JsonSchema, Deserialize, Serialize)]
// #[serde(remote = "Verbosity")]
// pub struct VerbosityDef<L = ErrorLevel>
// where
//     L: LogLevel,
// {
//     #[serde(getter = "Verbosity::verbose")]
//     verbose: u8,
//     #[serde(getter = "Verbosity::quiet")]
//     quiet: u8,
//     #[serde(skip)]
//     phantom: PhantomData<L>,
// }

// // You'd also need to implement From for deserialization
// impl<L: LogLevel> From<VerbosityDef<L>> for Verbosity<L> {
//     fn from(def: VerbosityDef<L>) -> Self {
//         // Use Verbosity's constructor methods
//         Verbosity::new(def.verbose, def.quiet)
//     }
// }

/// Simple program to greet a person
#[derive(Debug, Parser, PartialEq, Serialize, Deserialize, JsonSchema)]
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
    verbosity: Verbosity,
}

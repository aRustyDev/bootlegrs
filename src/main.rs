mod apps;
mod cli;
mod config;
mod schema;

use clap::{Arg, ArgAction, Command, Parser, ValueHint, builder::PossibleValue, value_parser};
use cli::args::Args;
use cli::commands::Bootleg;
use config::lib::Config;
use figment::{
    Figment,
    providers::{Env, Format, Json, Toml},
};
use serde::Deserialize;
use std::fmt;
use std::io;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
const PKG_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_VER: &str = env!("CARGO_PKG_VERSION");
const PKG_VER_MAJ: &str = env!("CARGO_PKG_VERSION_MAJOR");
const PKG_VER_MIN: &str = env!("CARGO_PKG_VERSION_MINOR");
const PKG_VER_PAT: &str = env!("CARGO_PKG_VERSION_PATCH");
const PKG_VER_PRE: &str = env!("CARGO_PKG_VERSION_PRE");

const OPT_DIR: &str = "/opt/bootleg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::load()?;

    println!("config: {:?}", config);

    match &args.command {
        Some(Bootleg::Completions { shell }) => {
            apps::completions::print(shell);
        }
        Some(Bootleg::Config { to }) => {
            todo!()
        }
        Some(Bootleg::Edit { to }) => {
            todo!()
        }
        Some(Bootleg::Health { to }) => {
            todo!()
        }
        Some(Bootleg::Info { to }) => {
            todo!()
        }
        Some(Bootleg::Init { to }) => {
            todo!()
        }
        Some(Bootleg::Install { to }) => {
            todo!()
        }
        Some(Bootleg::List { to }) => {
            todo!()
        }
        Some(Bootleg::New { to }) => {
            todo!()
        }
        Some(Bootleg::Schema { schema }) => {
            apps::schema::generate_schema(schema);
        }
        Some(Bootleg::Search { to }) => {
            todo!()
        }
        Some(Bootleg::Taps { to }) => {
            todo!()
        }
        Some(Bootleg::Tui { to }) => {
            todo!()
        }
        Some(Bootleg::Uninstall { to }) => {
            todo!()
        }
        Some(Bootleg::Update { to }) => {
            todo!()
        }
        Some(Bootleg::Upgrade { to }) => {
            todo!()
        }
        None => {
            println!("There was no subcommand given");
        }
    }
    Ok(())
}

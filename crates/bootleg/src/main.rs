use clap::Parser;
use args::Args;
use commands::Bootleg;
use schema::config::Config;

#[allow(unused)]
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[allow(unused)]
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
#[allow(unused)]
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
#[allow(unused)]
const PKG_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
#[allow(unused)]
const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
#[allow(unused)]
const PKG_REPO: &str = env!("CARGO_PKG_REPOSITORY");
#[allow(unused)]
const PKG_VER: &str = env!("CARGO_PKG_VERSION");
#[allow(unused)]
const PKG_VER_MAJ: &str = env!("CARGO_PKG_VERSION_MAJOR");
#[allow(unused)]
const PKG_VER_MIN: &str = env!("CARGO_PKG_VERSION_MINOR");
#[allow(unused)]
const PKG_VER_PAT: &str = env!("CARGO_PKG_VERSION_PATCH");
#[allow(unused)]
const PKG_VER_PRE: &str = env!("CARGO_PKG_VERSION_PRE");

#[allow(unused)]
const OPT_DIR: &str = "/opt/bootleg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::load()?;

    println!("config: {:?}", config);

    match &args.command {
        Some(Bootleg::Completions { shell }) => {
            completions::print(shell);
        }
        Some(Bootleg::Config { .. }) => {
            todo!()
        }
        Some(Bootleg::Edit { .. }) => {
            todo!()
        }
        Some(Bootleg::Health { .. }) => {
            todo!()
        }
        Some(Bootleg::Info { .. }) => {
            todo!()
        }
        Some(Bootleg::Init { .. }) => {
            todo!()
        }
        Some(Bootleg::Install { .. }) => {
            todo!()
        }
        Some(Bootleg::List { .. }) => {
            todo!()
        }
        Some(Bootleg::New { .. }) => {
            todo!()
        }
        Some(Bootleg::Schema { schema }) => {
            gen_schema::generate_schema(&schema);
        }
        Some(Bootleg::Search { .. }) => {
            todo!()
        }
        Some(Bootleg::Taps { .. }) => {
            todo!()
        }
        Some(Bootleg::Tui { .. }) => {
            todo!()
        }
        Some(Bootleg::Uninstall { .. }) => {
            todo!()
        }
        Some(Bootleg::Update { .. }) => {
            todo!()
        }
        Some(Bootleg::Upgrade { .. }) => {
            todo!()
        }
        None => {
            println!("There was no subcommand given");
        }
    }
    Ok(())
}

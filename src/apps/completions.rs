use crate::PKG_NAME;
use crate::cli::args::Args;
use clap::CommandFactory;
use clap_complete::aot::{Shell, generate};
use clap_complete_nushell::Nushell;
use std::io;

pub fn print(shell: &String) {
    let mut cmd = Args::command();
    match shell.to_lowercase().as_str() {
        "bash" => {
            generate(Shell::Bash, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        "elvish" => {
            generate(Shell::Elvish, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        "fish" => {
            generate(Shell::Fish, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        "powershell" => {
            generate(Shell::PowerShell, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        "zsh" => {
            generate(Shell::Zsh, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        "nu" => {
            generate(Nushell, &mut cmd, PKG_NAME, &mut io::stdout());
        }
        _ => {}
    }
}

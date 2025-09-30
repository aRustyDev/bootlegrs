const BIN_NAME: &str = "bootleg";
const MAN_DIR: &str = "./docs/man";

fn main() -> std::io::Result<()> {
    // Build MAN Pages at build time
    let out_dir =
        std::path::PathBuf::from(std::env::var_os(MAN_DIR).ok_or(std::io::ErrorKind::NotFound)?);

    let cmd = clap::Command::new(BIN_NAME)
        .arg(clap::arg!(-n --name <NAME>))
        .arg(clap::arg!(-c --count <NUM>));

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("{}.1", BIN_NAME), buffer)?;

    Ok(())
}

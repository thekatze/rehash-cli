use std::io::Read as _;

use clap::Parser as _;
use color_eyre::eyre::{Context, ContextCompat};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        help = "The url of the service to generate the password for, required to derive the password"
    )]
    store_file_path: String,

    #[arg(long, help = "Sets password to skip the interactive password prompt")]
    password: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut reader = std::io::BufReader::new(
        std::fs::File::open(args.store_file_path).context("File path not found")?,
    );

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .context("reading file as utf-8 failed")?;

    let vault_password = args
        .password
        .or_else(|| rpassword::prompt_password("Enter password: ").ok())
        .context("could not prompt password, expected interactive shell")?;

    let parsed = rehash_vault::load_from_json_string(&contents, &vault_password)
        .context("reading vault failed")?;

    dbg!(parsed);

    Ok(())
}

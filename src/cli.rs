use clap::Parser;

use crate::args::Args;
use crate::{config_dir, motd};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let config = config_dir()?.join("markotd.conf");

    if config.is_file() {
        dotenvy::from_path_override(config)?;
    }

    let out = motd::render(&args.template)?;

    println!("{out}");

    Ok(())
}

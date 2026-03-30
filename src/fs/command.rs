use std::process::Command;

use color_eyre::Result;

pub(crate) fn binary_exists_on_path(binary_name: &str) -> bool {
    which::which(binary_name).is_ok()
}

pub(crate) fn run_df() -> Result<String> {
    let output = String::from_utf8(
        Command::new("df").arg("-P").arg("-h").output()?.stdout,
    )?;

    Ok(output)
}

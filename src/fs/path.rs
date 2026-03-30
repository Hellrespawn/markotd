use camino::Utf8PathBuf;
use color_eyre::Result;
use color_eyre::eyre::eyre;

pub(crate) fn config_dir() -> Result<Utf8PathBuf> {
    Ok(home_dir()?.join(".markotd"))
}

fn home_dir() -> Result<Utf8PathBuf> {
    let path = dirs::home_dir().ok_or(eyre!("Unable to get home dir."))?;

    let utf8_path: Utf8PathBuf = path.try_into()?;

    Ok(utf8_path)
}

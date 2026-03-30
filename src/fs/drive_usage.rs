use color_eyre::Result;
use color_eyre::eyre::eyre;

use super::command::{binary_exists_on_path, run_df};
use super::filesystem::{Filesystem, FilesystemFilter};
use crate::Config;

pub(crate) fn drive_usage(config: &Config) -> Result<Vec<Filesystem>> {
    if !binary_exists_on_path("df") {
        return Err(eyre!("Unable to find `df` on path."));
    }

    let filter = FilesystemFilter::from_config(config)?;
    let output = run_df()?;

    let filesystems = output
        .lines()
        .skip(1)
        .map(Filesystem::from_df_line)
        .filter(|fs| fs.as_ref().is_ok_and(|fs| fs.filter_filesystem(&filter)))
        .collect::<Result<Vec<_>>>()?;

    Ok(filesystems)
}

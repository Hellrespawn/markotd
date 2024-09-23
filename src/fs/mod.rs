mod filesystem;
mod fs_max_length;
mod table;

use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use chrono::{DateTime, Local, NaiveDateTime};
use color_eyre::eyre::eyre;
use color_eyre::Result;
pub(crate) use filesystem::Filesystem;
pub(crate) use fs_max_length::FsMaxLength;
pub(crate) use table::FilesystemTable;

pub(crate) struct FsTools;

impl FsTools {
    pub(crate) fn binary_exists_on_path(binary_name: &str) -> bool {
        which::which(binary_name).is_ok()
    }

    pub(crate) fn get_last_update_time(
        path: &Utf8Path,
    ) -> Result<NaiveDateTime> {
        if !path.is_file() {
            return Err(eyre!("Path not found: {}", path));
        }

        let metadata = path.metadata()?;

        let mtime = metadata.modified()?;

        let datetime: DateTime<Local> = mtime.into();

        Ok(datetime.naive_local())
    }

    pub(crate) fn config() -> Result<Utf8PathBuf> {
        Ok(Self::home()?.join(".markotd"))
    }

    pub(crate) fn home() -> Result<Utf8PathBuf> {
        let path = dirs::home_dir().ok_or(eyre!("Unable to get home dir."))?;

        let utf8_path: Utf8PathBuf = path.try_into()?;

        Ok(utf8_path)
    }

    pub(crate) fn drive_usage() -> Result<Vec<Filesystem>> {
        assert!(
            FsTools::binary_exists_on_path("df"),
            "Unable to find `df` on path."
        );

        let output = String::from_utf8(
            Command::new("df").arg("-P").arg("-h").output()?.stdout,
        )?;

        let filesystems = output
            .lines()
            .skip(1)
            .filter_map(|fs| Filesystem::from_df_line(fs).transpose())
            .collect::<Result<Vec<_>>>()?;

        Ok(filesystems)
    }
}

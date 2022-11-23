mod filesystem;
mod fs_max_length;
mod table;

pub(crate) use filesystem::Filesystem;
pub(crate) use fs_max_length::FsMaxLength;
pub(crate) use table::FilesystemTable;

use chrono::{DateTime, Local, NaiveDateTime};
use std::path::{Path, PathBuf};

pub(crate) struct FsTools;

impl FsTools {
    pub(crate) fn binary_exists_on_path(binary_name: &str) -> bool {
        which::which(binary_name).is_ok()
    }

    pub(crate) fn get_last_update_time(path: &Path) -> NaiveDateTime {
        let metadata =
            path.metadata().expect("Unable to read metadata for file.");

        let mtime =
            metadata.modified().expect("Unable to read mtime for file.");

        let datetime: DateTime<Local> = mtime.into();

        datetime.naive_local()
    }

    pub(crate) fn home() -> PathBuf {
        dirs::home_dir().expect("Unable to get home dir.")
    }
}

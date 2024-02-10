mod filesystem;
mod fs_max_length;
mod table;

pub(crate) use filesystem::Filesystem;
pub(crate) use fs_max_length::FsMaxLength;
pub(crate) use table::FilesystemTable;

use crate::error::MarkotdError;
use crate::Result;
use chrono::{DateTime, Local, NaiveDateTime};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

pub(crate) struct FsTools;

impl FsTools {
    pub(crate) fn binary_exists_on_path(binary_name: &str) -> bool {
        which::which(binary_name).is_ok()
    }

    pub(crate) fn get_last_update_time(path: &Path) -> Result<NaiveDateTime> {
        if !path.is_file() {
            return Err(MarkotdError::NotFound(path.to_owned()));
        }

        let metadata =
            path.metadata().expect("Unable to read metadata for file.");

        let mtime =
            metadata.modified().expect("Unable to read mtime for file.");

        let datetime: DateTime<Local> = mtime.into();

        Ok(datetime.naive_local())
    }

    pub(crate) fn touch(path: &Path) {
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .expect("Unable to touch file.");
    }

    pub(crate) fn home() -> PathBuf {
        dirs::home_dir().expect("Unable to get home dir.")
    }

    pub(crate) fn tmp() -> PathBuf {
        std::env::temp_dir()
    }
}

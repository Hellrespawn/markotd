use std::process::Command;

use color_eyre::Result;

use super::{Module, ModuleFactory};
use crate::fs::{Filesystem, FilesystemTable, FsTools};

pub(crate) struct DriveUsage;

impl ModuleFactory for DriveUsage {
    fn create(&self) -> Result<Vec<Module>> {
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

        let table = FilesystemTable::new(filesystems);

        let body = table.to_string();

        Ok(vec![(Module::new("Drive Usage".to_owned(), body, 3))])
    }
}

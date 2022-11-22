use std::process::Command;

use crate::drive::{Filesystem, FilesystemTable};

use super::{Module, ModuleFactory};

pub(crate) struct DriveUsage;

impl ModuleFactory for DriveUsage {
    fn create(&self) -> Option<Module> {
        let output = String::from_utf8(
            Command::new("df")
                .arg("-h")
                .output()
                .expect("Unable to read filesystem data.")
                .stdout,
        )
        .expect("Unable to parse disk usage as UTF-8.");

        let filesystems = output
            .lines()
            .skip(1)
            .filter_map(Filesystem::from_df_line)
            .collect::<Vec<_>>();

        let table = FilesystemTable::new(filesystems);

        let body = table.to_string();

        Some(Module::new("Drive Usage".to_owned(), body, 3))
    }
}

impl DriveUsage {}

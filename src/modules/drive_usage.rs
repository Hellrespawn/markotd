use crate::drive::{Filesystem, FilesystemTable};

use super::{Module, ModuleFactory};
use once_cell::sync::Lazy;
use regex::Regex;
use systemstat::{Filesystem as SystemStatFilesystem, Platform, System};

static FS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([[:alpha:]]:|/dev)").expect("Unable to compile regex.")
});

pub(crate) struct DriveUsage;

impl ModuleFactory for DriveUsage {
    fn create(&self) -> Option<Module> {
        let filesystems = System::new()
            .mounts()
            .expect("Unable to read mounted filesystems.")
            .into_iter()
            .filter(DriveUsage::filter_filesystem)
            .map(Filesystem::from_system_stat_filesystem)
            .collect::<Vec<_>>();

        let table = FilesystemTable::new(filesystems);

        let body = table.to_string();

        Some(Module::new("Drive Usage".to_owned(), body, 3))
    }
}

impl DriveUsage {
    fn filter_filesystem(filesystem: &SystemStatFilesystem) -> bool {
        FS_REGEX.is_match(&filesystem.fs_mounted_from)
            && !filesystem.fs_mounted_from.contains("docker")
    }
}

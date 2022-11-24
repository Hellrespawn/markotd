use std::path::{Path, PathBuf};

use chrono::{Duration, Local, NaiveDateTime};

use crate::{Config, DateTime, FsTools};

use super::{Module, ModuleFactory};

pub(crate) struct UpdateCheck;

impl ModuleFactory for UpdateCheck {
    fn create(&self) -> Option<Module> {
        if FsTools::binary_exists_on_path("apt") {
            let apt_cache_file = PathBuf::from("/var/cache/apt/pkgcache.bin");

            UpdateCheck::last_updated_time_apt(&apt_cache_file)
        } else if FsTools::binary_exists_on_path("pacman") {
            let file_contents = UpdateCheck::read_pacman_log();
            UpdateCheck::last_updated_time_pacman(&file_contents)
        } else {
            None
        }
    }
}

impl UpdateCheck {
    fn last_updated_time_apt(apt_cache_file: &Path) -> Option<Module> {
        if !apt_cache_file.is_file() {
            return None;
        }

        let last_update_time = FsTools::get_last_update_time(apt_cache_file)
            .unwrap_or_else(|e| panic!("{}", e));

        UpdateCheck::create_module("APT", last_update_time)
    }

    fn last_updated_time_pacman(pacman_log: &str) -> Option<Module> {
        let line = pacman_log
            .lines()
            .rev()
            .find(|l| l.contains("starting full system upgrade"))
            .map(|string| {
                let date = &string.split_whitespace().next().expect("Unable to extract date. Has the pacman log format changed?");
                &date[1..date.len() - 1]
            });

        if let Some(string) = line {
            // 2022-11-17T19:18:01+0100
            let fmt = "%Y-%m-%dT%H:%M:%S%z";

            let last_update_time = NaiveDateTime::parse_from_str(string, fmt)
                .expect("Unable to parse date from pacman log file.");

            UpdateCheck::create_module("pacman", last_update_time)
        } else {
            None
        }
    }

    fn read_pacman_log() -> String {
        let file = PathBuf::from("/var/log/pacman.log");

        std::fs::read_to_string(file).unwrap_or_default()
    }

    fn create_module(
        name: &str,
        last_update_time: NaiveDateTime,
    ) -> Option<Module> {
        let duration = Local::now().naive_local() - last_update_time;

        if duration < Config::notify_update_after() {
            None
        } else {
            Some(Module::new(
                UpdateCheck::title(name, last_update_time),
                UpdateCheck::body(duration),
                2,
            ))
        }
    }

    fn title(name: &str, last_update_time: NaiveDateTime) -> String {
        format!(
            "{} last updated on {}",
            name,
            DateTime::format_date(last_update_time)
        )
    }

    fn body(duration: Duration) -> String {
        let seconds: u64 = duration
            .num_seconds()
            .try_into()
            .expect("Negative duration since last update!");

        format!("It has been {}.", DateTime::format_duration(seconds))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    fn set_file_mtime(path: &Path) {
        let now = Local::now();

        // 10 days ago
        let seconds = 10 * 24 * 60 * 60;

        let filetime =
            filetime::FileTime::from_unix_time(now.timestamp() - seconds, 0);

        filetime::set_file_mtime(path, filetime)
            .expect("Unable to set temporary file mtime.");
    }

    #[test]
    fn test_apt() {
        let test_dir =
            TempDir::new().expect("Unable to create temporary directory.");

        let test_file = test_dir.child("test_file");
        test_file.touch().expect("Unable to create test file.");

        assert!(test_file.path().is_file());

        set_file_mtime(test_file.path());

        UpdateCheck::last_updated_time_apt(test_file.path())
            .expect("Could not create module from test file.");
    }

    #[test]
    fn test_pacman() {
        let test_data = include_str!("../../test/pacman.log");

        let module = UpdateCheck::last_updated_time_pacman(test_data)
            .expect("Could not create module from test data.");

        println!("{}", module);
    }
}

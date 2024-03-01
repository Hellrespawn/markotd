use camino::{Utf8Path, Utf8PathBuf};
use chrono::{Duration, Local, NaiveDateTime};
use color_eyre::Result;

use super::{Module, ModuleFactory};
use crate::{Config, DateTime, FsTools};

pub(crate) struct UpdateCheck;

impl ModuleFactory for UpdateCheck {
    fn create(&self) -> Result<Vec<Module>> {
        if FsTools::binary_exists_on_path("apt") {
            let apt_cache_file =
                Utf8PathBuf::from("/var/cache/apt/pkgcache.bin");

            Self::last_updated_time_apt(&apt_cache_file)
        } else if FsTools::binary_exists_on_path("pacman") {
            let file_contents = Self::read_pacman_log()?;

            Self::last_updated_time_pacman(&file_contents)
        } else {
            Ok(vec![])
        }
    }
}

impl UpdateCheck {
    fn last_updated_time_apt(apt_cache_file: &Utf8Path) -> Result<Vec<Module>> {
        if !apt_cache_file.is_file() {
            return Ok(vec![]);
        }

        let last_update_time = FsTools::get_last_update_time(apt_cache_file)?;

        UpdateCheck::create_module("APT", last_update_time)
    }

    fn last_updated_time_pacman(pacman_log: &str) -> Result<Vec<Module>> {
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
                .expect("Unable to parse date from pacman log file. Has the pacman log format changed?");

            UpdateCheck::create_module("pacman", last_update_time)
        } else {
            Ok(vec![])
        }
    }

    fn read_pacman_log() -> Result<String> {
        let file = Utf8PathBuf::from("/var/log/pacman.log");

        Ok(fs_err::read_to_string(file)?)
    }

    fn create_module(
        name: &str,
        last_update_time: NaiveDateTime,
    ) -> Result<Vec<Module>> {
        let duration = Local::now().naive_local() - last_update_time;

        if duration < Config::notify_update_after()? {
            Ok(vec![])
        } else {
            Ok(vec![Module::new(
                UpdateCheck::title(name, last_update_time),
                UpdateCheck::body(duration)?,
                2,
            )])
        }
    }

    fn title(name: &str, last_update_time: NaiveDateTime) -> String {
        format!(
            "{} last updated on {}",
            name,
            DateTime::format_date(last_update_time)
        )
    }

    fn body(duration: Duration) -> Result<String> {
        let seconds: u64 = duration
            .num_seconds()
            .try_into()
            .expect("Negative duration since last update!");

        Ok(format!("It has been {}.", DateTime::format_duration(seconds)?))
    }
}

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    use super::*;

    fn set_file_mtime(path: &Utf8Path) {
        let now = Local::now();

        // 10 days ago
        let seconds = 10 * 24 * 60 * 60;

        let filetime =
            filetime::FileTime::from_unix_time(now.timestamp() - seconds, 0);

        filetime::set_file_mtime(path, filetime)
            .expect("Unable to set temporary file mtime.");
    }

    #[test]
    fn test_apt() -> Result<()> {
        let test_dir =
            TempDir::new().expect("Unable to create temporary directory.");

        let test_file = test_dir.child("test_file");
        test_file.touch().expect("Unable to create test file.");

        assert!(test_file.path().is_file());

        let path: Utf8PathBuf =
            Utf8PathBuf::try_from(test_file.path().to_path_buf()).unwrap();

        set_file_mtime(&path);

        assert!(!UpdateCheck::last_updated_time_apt(&path)?.is_empty());

        Ok(())
    }

    #[test]
    fn test_pacman() -> Result<()> {
        let test_data = include_str!("../../test/pacman.log");

        assert!(!UpdateCheck::last_updated_time_pacman(test_data)?.is_empty());

        Ok(())
    }
}

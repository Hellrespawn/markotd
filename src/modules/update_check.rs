use std::path::PathBuf;

use chrono::{Duration, Local, NaiveDateTime};

use crate::{DateTimeTools, FsTools};

use super::{Module, ModuleFactory};

pub(crate) struct UpdateCheck;

impl ModuleFactory for UpdateCheck {
    fn create(&self) -> Option<Module> {
        if FsTools::binary_exists_on_path("apt") {
            UpdateCheck::last_updated_time_apt()
        } else if FsTools::binary_exists_on_path("pacman") {
            UpdateCheck::last_updated_time_pacman()
        } else {
            None
        }
    }
}

impl UpdateCheck {
    fn last_updated_time_apt() -> Option<Module> {
        let file = PathBuf::from("/var/cache/apt/pkgcache.bin");

        if !file.is_file() {
            return None;
        }

        let last_update_time = FsTools::get_last_update_time(&file);

        Some(UpdateCheck::create_module("APT", last_update_time))
    }

    fn last_updated_time_pacman() -> Option<Module> {
        let file = PathBuf::from("/var/log/pacman.log");

        if !file.is_file() {
            return None;
        }

        let file_contents =
            std::fs::read_to_string(&file).expect("Unable to read pacman log.");

        let line = file_contents
            .lines()
            .rev()
            .find(|l| l.contains("starting full system upgrade"))
            .map(|string| {
                &string.split_whitespace().next().expect("Unable to extract date. Has the pacman log format changed?")[1..string.len() - 1]
            });

        if let Some(string) = line {
            // 2022-11-17T19:18:01+0100
            let fmt = "%Y-%m-%dT%H:%M:%S%z";

            let last_update_time = NaiveDateTime::parse_from_str(string, fmt)
                .expect("Unable to parse date from pacman log file.");

            Some(UpdateCheck::create_module("pacman", last_update_time))
        } else {
            None
        }
    }

    fn create_module(name: &str, last_update_time: NaiveDateTime) -> Module {
        let duration = Local::now().naive_local() - last_update_time;

        Module::new(
            UpdateCheck::title(name, last_update_time),
            UpdateCheck::body(duration),
            2,
        )
    }

    fn title(name: &str, last_update_time: NaiveDateTime) -> String {
        format!(
            "{} last updated on {}",
            name,
            DateTimeTools::format_date(last_update_time)
        )
    }

    fn body(duration: Duration) -> String {
        let seconds: u64 = duration
            .num_seconds()
            .try_into()
            .expect("Negative duration since last update!");

        format!("It has been {}.", DateTimeTools::format_duration(seconds))
    }
}

use camino::{Utf8Path, Utf8PathBuf};
use chrono::{Local, NaiveDateTime};
use color_eyre::Result;
use serde::Serialize;

use crate::{Config, DateTime, FsTools};

#[derive(Serialize)]
pub struct LastUpdated {
    time_since: String,
    date: String,
    app: String,
}

pub fn get_last_update_time() -> Result<Option<LastUpdated>> {
    if FsTools::binary_exists_on_path("apt") {
        let apt_cache_file = Utf8PathBuf::from("/var/cache/apt/pkgcache.bin");

        last_updated_time_apt(&apt_cache_file)
    } else if FsTools::binary_exists_on_path("pacman") {
        let file_contents = read_pacman_log()?;

        last_updated_time_pacman(&file_contents)
    } else {
        Ok(None)
    }
}

fn read_pacman_log() -> Result<String> {
    let file = Utf8PathBuf::from("/var/log/pacman.log");

    Ok(fs_err::read_to_string(file)?)
}

fn last_updated_time_apt(
    apt_cache_file: &Utf8Path,
) -> Result<Option<LastUpdated>> {
    if !apt_cache_file.is_file() {
        return Ok(None);
    }

    let last_update_time = FsTools::get_last_update_time(apt_cache_file)?;

    create_datetime("APT", last_update_time)
}

fn last_updated_time_pacman(pacman_log: &str) -> Result<Option<LastUpdated>> {
    let line = pacman_log
        .lines()
        .rev()
        .find(|l| l.contains("starting full system upgrade"))
        .map(|string| {
            let date = &string.split_whitespace().next().expect(
                "Unable to extract date. Has the pacman log format changed?",
            );
            &date[1..date.len() - 1]
        });

    if let Some(string) = line {
        // 2022-11-17T19:18:01+0100
        let fmt = "%Y-%m-%dT%H:%M:%S%z";

        let last_update_time = NaiveDateTime::parse_from_str(string, fmt)
            .expect("Unable to parse date from pacman log file. Has the pacman log format changed?");

        create_datetime("pacman", last_update_time)
    } else {
        Ok(None)
    }
}

fn create_datetime(
    name: &str,
    last_update_time: NaiveDateTime,
) -> Result<Option<LastUpdated>> {
    let duration = Local::now().naive_local() - last_update_time;

    if duration < Config::notify_update_after()? {
        Ok(None)
    } else {
        Ok(Some(LastUpdated {
            time_since: DateTime::format_duration(
                duration.num_seconds().try_into()?,
            )?,
            date: DateTime::format_date(last_update_time),
            app: name.to_owned(),
        }))
    }
}

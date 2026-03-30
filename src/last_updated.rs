use camino::{Utf8Path, Utf8PathBuf};
use chrono::{Local, NaiveDateTime};
use color_eyre::Result;
use color_eyre::eyre::eyre;

use crate::{
    Config, DateTime, LastUpdated, binary_exists_on_path, modified_time,
};

pub fn get_last_update_time(config: &Config) -> Result<Option<LastUpdated>> {
    if binary_exists_on_path("apt") {
        let apt_cache_file = Utf8PathBuf::from("/var/cache/apt/pkgcache.bin");

        last_updated_time_apt(config, &apt_cache_file)
    } else if binary_exists_on_path("pacman") {
        let file_contents = read_pacman_log()?;

        last_updated_time_pacman(config, &file_contents)
    } else {
        Ok(None)
    }
}

fn read_pacman_log() -> Result<String> {
    let file = Utf8PathBuf::from("/var/log/pacman.log");

    Ok(fs_err::read_to_string(file)?)
}

fn last_updated_time_apt(
    config: &Config,
    apt_cache_file: &Utf8Path,
) -> Result<Option<LastUpdated>> {
    if !apt_cache_file.is_file() {
        return Ok(None);
    }

    let last_update_time = modified_time(apt_cache_file)?;

    create_datetime(config, "APT", last_update_time)
}

fn last_updated_time_pacman(
    config: &Config,
    pacman_log: &str,
) -> Result<Option<LastUpdated>> {
    let line = pacman_log
        .lines()
        .rev()
        .find(|line| line.contains("starting full system upgrade"));

    let Some(line) = line else {
        return Ok(None);
    };

    let last_update_time = parse_pacman_upgrade_time(line)?;

    create_datetime(config, "pacman", last_update_time)
}

fn parse_pacman_upgrade_time(line: &str) -> Result<NaiveDateTime> {
    let raw_timestamp = line.split_whitespace().next().ok_or_else(|| {
        eyre!("Unable to extract date from pacman log line: {line}")
    })?;

    if !raw_timestamp.starts_with('[') || !raw_timestamp.ends_with(']') {
        return Err(eyre!(
            "Unable to parse pacman log timestamp: {raw_timestamp}"
        ));
    }

    let timestamp = &raw_timestamp[1..raw_timestamp.len() - 1];
    let fmt = "%Y-%m-%dT%H:%M:%S%z";

    NaiveDateTime::parse_from_str(timestamp, fmt).map_err(|err| {
        eyre!("Unable to parse pacman log date '{timestamp}': {err}")
    })
}

fn create_datetime(
    config: &Config,
    name: &str,
    last_update_time: NaiveDateTime,
) -> Result<Option<LastUpdated>> {
    let duration = Local::now().naive_local() - last_update_time;

    if duration < config.notify_update_after() {
        Ok(None)
    } else {
        Ok(Some(LastUpdated::new(
            DateTime::format_duration(
                duration.num_seconds().try_into()?,
                config,
            )?,
            DateTime::format_date(last_update_time),
            name.to_owned(),
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_last_updated_time_pacman_parses_fixture() -> Result<()> {
        let config = Config::default();
        let pacman_log = include_str!("../test/pacman.log");

        let last_updated = last_updated_time_pacman(&config, pacman_log)?;

        assert!(last_updated.is_some());

        Ok(())
    }

    #[test]
    fn test_last_updated_time_pacman_rejects_invalid_timestamp() {
        let config = Config::default();
        let pacman_log =
            "[invalid-timestamp] [PACMAN] starting full system upgrade";

        let Err(err) = last_updated_time_pacman(&config, pacman_log) else {
            panic!("expected error");
        };

        assert!(err.to_string().contains("Unable to parse pacman log"));
    }
}

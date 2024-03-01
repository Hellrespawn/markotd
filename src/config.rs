use camino::Utf8PathBuf;
use chrono::Duration;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use itertools::Itertools;

pub(crate) struct Config;

impl Config {
    pub(crate) fn notify_update_after() -> Result<Duration> {
        if let Ok(string) = std::env::var("NOTIFY_UPDATE_HOURS") {
            let hours: i64 = string.parse()?;

            Ok(Duration::hours(hours))
        } else {
            Ok(Duration::days(3))
        }
    }

    pub(crate) fn duration_divisions() -> Result<usize> {
        if let Ok(string) = std::env::var("DUR_DIV") {
            let divisions: usize = string.parse()?;

            Ok(divisions)
        } else {
            Ok(3)
        }
    }

    pub(crate) fn watched_files() -> Result<Vec<Utf8PathBuf>> {
        if let Ok(string) = std::env::var("WATCH_FILES") {
            let paths =
                string.split(';').map(Utf8PathBuf::from).collect::<Vec<_>>();

            let (existing, not_existing): (Vec<_>, Vec<_>) =
                paths.into_iter().partition(|p| p.is_file());

            if !not_existing.is_empty() {
                return Err(eyre!(
                    "Cannot read the following files:\n{}",
                    not_existing
                        .into_iter()
                        .map(|p| p.to_string())
                        .intersperse("\n".to_owned())
                        .collect::<String>()
                ));
            }

            Ok(existing)
        } else {
            Ok(vec![])
        }
    }
}

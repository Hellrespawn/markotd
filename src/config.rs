use chrono::Duration;

pub(crate) struct Config;

impl Config {
    pub(crate) fn notify_update_after() -> Duration {
        if let Some(string) = option_env!("NOTIFY_UPDATE_HOURS") {
            Duration::hours(string.parse().expect("Unable to parse duration."))
        } else {
            Duration::days(3)
        }
    }

    pub(crate) fn show_every_hours() -> Duration {
        if let Some(string) = option_env!("SHOW_EVERY_HOURS") {
            Duration::hours(string.parse().expect("Unable to parse duration."))
        } else {
            Duration::hours(0)
        }
    }
}

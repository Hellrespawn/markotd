use chrono::Duration;
use color_eyre::Result;
use color_eyre::eyre::{WrapErr, eyre};
use regex::Regex;

use crate::date_time::MAX_DIVISIONS;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Config {
    notify_update_after: Duration,
    duration_divisions: usize,
    df_whitelist_regex: Option<String>,
    df_blacklist_regex: Option<String>,
}

impl Config {
    pub(crate) fn load() -> Result<Self> {
        let notify_update_after = if let Ok(string) =
            std::env::var("NOTIFY_UPDATE_HOURS")
        {
            let hours: i64 =
                string.parse().wrap_err("Invalid NOTIFY_UPDATE_HOURS value")?;

            Duration::hours(hours)
        } else {
            Duration::days(3)
        };

        let duration_divisions = if let Ok(string) = std::env::var("DUR_DIV") {
            string.parse().wrap_err("Invalid DUR_DIV value")?
        } else {
            3
        };

        if duration_divisions > MAX_DIVISIONS {
            return Err(eyre!(
                "Invalid DUR_DIV value: {} exceeds maximum of {}",
                duration_divisions,
                MAX_DIVISIONS
            ));
        }

        let df_whitelist_regex = std::env::var("DF_WHITELIST").ok();
        let df_blacklist_regex = std::env::var("DF_BLACKLIST").ok();

        validate_regex("DF_WHITELIST", df_whitelist_regex.as_deref())?;
        validate_regex("DF_BLACKLIST", df_blacklist_regex.as_deref())?;

        Ok(Self {
            notify_update_after,
            duration_divisions,
            df_whitelist_regex,
            df_blacklist_regex,
        })
    }

    pub(crate) fn notify_update_after(&self) -> Duration {
        self.notify_update_after
    }

    pub(crate) fn duration_divisions(&self) -> usize {
        self.duration_divisions
    }

    pub(crate) fn df_whitelist_regex(&self) -> Option<&str> {
        self.df_whitelist_regex.as_deref()
    }

    pub(crate) fn df_blacklist_regex(&self) -> Option<&str> {
        self.df_blacklist_regex.as_deref()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            notify_update_after: Duration::days(3),
            duration_divisions: 3,
            df_whitelist_regex: None,
            df_blacklist_regex: None,
        }
    }
}

fn validate_regex(name: &str, regex: Option<&str>) -> Result<()> {
    if let Some(regex) = regex {
        Regex::new(regex).wrap_err_with(|| format!("Invalid {name} regex"))?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::sync::{LazyLock, Mutex};

    use super::*;

    static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn clear_config_env() {
        unsafe {
            std::env::remove_var("DUR_DIV");
            std::env::remove_var("NOTIFY_UPDATE_HOURS");
            std::env::remove_var("DF_WHITELIST");
            std::env::remove_var("DF_BLACKLIST");
        }
    }

    #[test]
    fn test_load_uses_defaults() -> Result<()> {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_config_env();

        let config = Config::load()?;

        assert_eq!(config, Config::default());

        Ok(())
    }

    #[test]
    fn test_load_reads_valid_env_vars() -> Result<()> {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_config_env();
        unsafe {
            std::env::set_var("NOTIFY_UPDATE_HOURS", "12");
            std::env::set_var("DUR_DIV", "2");
            std::env::set_var("DF_WHITELIST", "^/srv");
            std::env::set_var("DF_BLACKLIST", "tmpfs");
        }

        let config = Config::load()?;

        assert_eq!(config.notify_update_after(), Duration::hours(12));
        assert_eq!(config.duration_divisions(), 2);
        assert_eq!(config.df_whitelist_regex(), Some("^/srv"));
        assert_eq!(config.df_blacklist_regex(), Some("tmpfs"));

        clear_config_env();

        Ok(())
    }

    #[test]
    fn test_load_rejects_invalid_duration_divisions() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_config_env();
        unsafe {
            std::env::set_var("DUR_DIV", "99");
        }

        let err = Config::load().unwrap_err();

        assert!(err.to_string().contains("Invalid DUR_DIV value"));

        clear_config_env();
    }

    #[test]
    fn test_load_rejects_invalid_whitelist_regex() {
        let _guard = ENV_LOCK.lock().unwrap();
        clear_config_env();
        unsafe {
            std::env::set_var("DF_WHITELIST", "[");
        }

        let err = Config::load().unwrap_err();

        assert!(err.to_string().contains("Invalid DF_WHITELIST regex"));

        clear_config_env();
    }
}

use chrono::NaiveDateTime;
use systemstat::{saturating_sub_bytes, Platform, System as SystemStat};

use crate::{Date, Misc};

pub(crate) struct System;

impl System {
    pub(crate) fn get_hostname() -> String {
        hostname::get()
            .expect("Unable to read hostname.")
            .to_str()
            .expect("Hostname is not valid UTF-8.")
            .to_string()
    }

    pub(crate) fn get_username() -> String {
        whoami::username()
    }

    pub(crate) fn get_platform_name() -> String {
        whoami::distro()
    }

    pub(crate) fn get_uptime() -> String {
        let uptime =
            SystemStat::new().uptime().expect("Unable to read uptime.");

        Date::format_duration(uptime)
    }

    pub(crate) fn get_boot_time() -> String {
        let boot_time = SystemStat::new()
            .boot_time()
            .expect("Unable to read boot time.");

        let seconds = boot_time.unix_timestamp();

        let date_time = NaiveDateTime::from_timestamp_opt(seconds, 0)
            .expect("Unable to interpret boot time as NaiveDateTime.");

        let utc =
            chrono::DateTime::<chrono::Utc>::from_utc(date_time, chrono::Utc);

        let local: chrono::DateTime<chrono::Local> =
            chrono::DateTime::from(utc);

        Date::format_date(local.naive_local())
    }

    pub(crate) fn get_memory_usage() -> String {
        let memory = SystemStat::new()
            .memory()
            .expect("Unable to read memory info.");

        let total = memory.total;

        let used = saturating_sub_bytes(memory.total, memory.free);

        format!(
            "{} of {} ({:05.2}%) RAM is in use.",
            used,
            total,
            Misc::pct_from_byte_sizes(used, total)
        )
    }
}

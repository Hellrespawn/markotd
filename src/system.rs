use crate::DateTime;
use chrono::{DateTime as ChronoDateTime, Local, NaiveDateTime, Utc};
use systemstat::{
    saturating_sub_bytes, ByteSize, Platform, System as SystemStat,
};

pub(crate) struct System;

impl System {
    pub(crate) fn username() -> String {
        whoami::username()
    }

    pub(crate) fn platform_name() -> String {
        whoami::distro()
    }

    pub(crate) fn hostname() -> String {
        hostname::get()
            .expect("Unable to read hostname.")
            .to_str()
            .expect("Hostname is not valid UTF-8.")
            .to_string()
    }

    pub(crate) fn uptime() -> String {
        let uptime =
            SystemStat::new().uptime().expect("Unable to read uptime.");

        DateTime::format_duration(uptime.as_secs())
    }

    pub(crate) fn boot_time() -> String {
        let boot_time = SystemStat::new()
            .boot_time()
            .expect("Unable to read boot time.");

        let seconds = boot_time.unix_timestamp();

        let date_time = NaiveDateTime::from_timestamp_opt(seconds, 0)
            .expect("Unable to interpret boot time as NaiveDateTime.");

        let utc = ChronoDateTime::<Utc>::from_utc(date_time, Utc);

        let local: ChronoDateTime<Local> = chrono::DateTime::from(utc);

        DateTime::format_date(local.naive_local())
    }

    pub(crate) fn memory_usage() -> String {
        let memory = SystemStat::new()
            .memory()
            .expect("Unable to read memory info.");

        let total = memory.total;

        let used = saturating_sub_bytes(memory.total, memory.free);

        format!(
            "{} of {} ({:05.2}%) RAM is in use.",
            used,
            total,
            System::pct_from_byte_sizes(used, total)
        )
    }

    fn pct_from_byte_sizes(used: ByteSize, total: ByteSize) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let pct: f64 = (used.as_u64() as f64 / total.as_u64() as f64) * 100.0;

        pct
    }
}

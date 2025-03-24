use chrono::{DateTime as ChronoDateTime, Local, Utc};
use color_eyre::Result;
use color_eyre::eyre::eyre;
use systemstat::{
    ByteSize, Platform, System as SystemStat, saturating_sub_bytes,
};

use crate::DateTime;
use crate::template::Ram;

pub(crate) struct System;

impl System {
    pub(crate) fn username() -> String {
        whoami::username()
    }

    pub(crate) fn platform_name() -> String {
        whoami::distro()
    }

    pub(crate) fn hostname() -> Result<String> {
        Ok(hostname::get()?
            .to_str()
            .ok_or(eyre!("Hostname is not valid UTF-8."))?
            .to_string())
    }

    pub(crate) fn uptime() -> Result<String> {
        let uptime = SystemStat::new().uptime()?;

        DateTime::format_duration(uptime.as_secs())
    }

    pub(crate) fn boot_time() -> Result<String> {
        let boot_time = SystemStat::new().boot_time()?;

        let seconds = boot_time.unix_timestamp();

        let utc = ChronoDateTime::<Utc>::from_timestamp(seconds, 0)
            .ok_or(eyre!("Unable to parse timestamp"))?;

        let local: ChronoDateTime<Local> = chrono::DateTime::from(utc);

        Ok(DateTime::format_date(local.naive_local()))
    }

    pub(crate) fn memory_usage() -> Result<Ram> {
        let memory = SystemStat::new().memory()?;

        let total = memory.total;

        let used = saturating_sub_bytes(memory.total, memory.free);

        // {:5.2}

        Ok(Ram::new(
            used.to_string(),
            format!("{:2.0}", System::pct_from_byte_sizes(used, total)),
            total.to_string(),
        ))
    }

    fn pct_from_byte_sizes(used: ByteSize, total: ByteSize) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let pct: f64 = (used.as_u64() as f64 / total.as_u64() as f64) * 100.0;

        pct
    }
}

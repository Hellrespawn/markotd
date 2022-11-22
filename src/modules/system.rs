use chrono::Local;
use systemstat::{Duration, OffsetDateTime, Platform, System};

use super::Module;

pub(crate) fn create() -> Option<Module> {
    let now = Local::now();

    let title = format!("System status at {}", now);

    let system = System::new();

    let uptime_string = format!(
        "{} is up for {}, since {:?}",
        whoami::hostname(),
        get_uptime(&system),
        get_boottime(&system)
    );

    Some(Module::new(title, uptime_string))
}

fn get_uptime(system: &System) -> String {
    let uptime = system.uptime().expect("Unable to read uptime");

    format_duration(uptime)
}

fn format_duration(duration: Duration) -> String {
    let duration = duration.as_secs();

    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60 / 60) % 60;
    let days = duration / 60 / 60 / 24;

    let mut strings = Vec::new();

    if seconds > 0 {
        strings.push(format!("{} seconds", seconds))
    }

    if minutes > 0 {
        strings.push(format!("{} minutes", minutes))
    }

    if hours > 0 {
        strings.push(format!("{} hours", hours))
    }

    if days > 0 {
        strings.push(format!("{} days", days))
    }

    strings.reverse();

    strings.join(", ")
}

fn get_boottime(system: &System) -> OffsetDateTime {
    system.boot_time().expect("Unable to boot time")
}

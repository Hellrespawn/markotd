use crate::{Date, System};
use chrono::Local;

use super::{Module, ModuleFactory};

pub(crate) struct Status;

impl ModuleFactory for Status {
    fn create(&self) -> Option<Module> {
        let now = Local::now();

        let title = format!(
            "System status at {}",
            Date::format_date(now.naive_local())
        );

        let uptime_string = format!(
            "{} is up for {},\nsince {}.",
            System::get_hostname(),
            System::get_uptime(),
            System::get_boot_time()
        );

        let memory_string = System::get_memory_usage();

        let body = [uptime_string, memory_string].join("\n\n");

        Some(Module::new(title, body, 2))
    }
}

use crate::{DateTimeTools, SystemTools};
use chrono::Local;

use super::{Module, ModuleFactory};

pub(crate) struct Status;

impl ModuleFactory for Status {
    fn create(&self) -> Option<Module> {
        let now = Local::now();

        let title = format!(
            "System status at {}",
            DateTimeTools::format_date(now.naive_local())
        );

        let uptime_string = format!(
            "{} is up for {},\nsince {}.",
            SystemTools::get_hostname(),
            SystemTools::get_uptime(),
            SystemTools::get_boot_time()
        );

        let memory_string = SystemTools::get_memory_usage();

        let body = [uptime_string, memory_string].join("\n\n");

        Some(Module::new(title, body, 2))
    }
}

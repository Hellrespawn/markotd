use super::{Module, ModuleFactory};
use crate::{DateTime, System};

pub(crate) struct Status;

impl ModuleFactory for Status {
    fn create(&self) -> Option<Module> {
        let now = DateTime::now();

        let title = format!("System status at {}", DateTime::format_date(now));

        let uptime_string = format!(
            "{} is up for {}, since {}.",
            System::hostname(),
            System::uptime(),
            System::boot_time()
        );

        let memory_string = System::memory_usage();

        let body = [uptime_string, memory_string].join("\n\n");

        Some(Module::new(title, body, 2))
    }
}

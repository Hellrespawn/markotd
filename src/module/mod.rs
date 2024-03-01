use colored::Colorize;

mod drive_usage;
mod file_watch;
mod header;
mod links;
mod status;
mod update_check;

pub(crate) fn get_module_factories() -> Vec<Box<dyn ModuleFactory>> {
    vec![
        Box::new(header::Header),
        Box::new(links::Links),
        Box::new(status::Status),
        Box::new(drive_usage::DriveUsage),
        Box::new(update_check::UpdateCheck),
        Box::new(file_watch::FileWatch),
    ]
}

pub(crate) trait ModuleFactory {
    fn create(&self) -> color_eyre::Result<Vec<Module>>;
}

#[derive(Debug)]
pub(crate) struct Module {
    title: String,
    body: String,
    heading_depth: usize,
}

impl Module {
    pub(crate) fn new(
        title: String,
        body: String,
        heading_depth: usize,
    ) -> Self {
        Self { title, body, heading_depth }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}\n\n{}",
            "#".repeat(self.heading_depth).color("cyan"),
            self.title.color("cyan"),
            self.body
        )
    }
}

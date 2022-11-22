use colored::Colorize;

mod header;
mod links;

type ModuleFactory = fn() -> Option<Module>;

pub(crate) fn get_module_factories() -> &'static [ModuleFactory] {
    &[header::create_header, links::create_links]
}

pub(crate) struct Module {
    title: String,
    body: String,
}

impl Module {
    pub(crate) fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n\n{}", self.title.color("cyan"), self.body)
    }
}

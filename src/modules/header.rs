use super::{Module, ModuleFactory};
use crate::SystemTools;

pub(crate) struct Header;

impl ModuleFactory for Header {
    fn create(&self) -> Option<Module> {
        let title = format!(
            "{} on {}",
            SystemTools::get_platform_name(),
            SystemTools::get_hostname()
        );

        let body =
            format!("The current user is {}.", SystemTools::get_username());

        Some(Module::new(title, body, 1))
    }
}

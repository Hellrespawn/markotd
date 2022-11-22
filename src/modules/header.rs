use super::{Module, ModuleFactory};
use crate::System;

pub(crate) struct Header;

impl ModuleFactory for Header {
    fn create(&self) -> Option<Module> {
        let title = format!(
            "{} on {}",
            System::get_platform_name(),
            System::get_hostname()
        );

        let body = format!("The current user is {}.", System::get_username());

        Some(Module::new(title, body, 1))
    }
}

use super::{Module, ModuleFactory};
use crate::System;

pub(crate) struct Header;

impl ModuleFactory for Header {
    fn create(&self) -> Option<Module> {
        let title =
            format!("{} on {}", System::platform_name(), System::hostname());

        let body = format!("The current user is {}.", System::username());

        Some(Module::new(title, body, 1))
    }
}

impl Header {}

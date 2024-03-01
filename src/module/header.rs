use color_eyre::Result;

use super::{Module, ModuleFactory};
use crate::System;

pub(crate) struct Header;

impl ModuleFactory for Header {
    fn create(&self) -> Result<Vec<Module>> {
        let title =
            format!("{} on {}", System::platform_name(), System::hostname()?);

        let body = format!("The current user is {}.", System::username());

        Ok(vec![(Module::new(title, body, 1))])
    }
}

impl Header {
}

use color_eyre::Result;

use super::{Module, ModuleFactory};
use crate::Config;

pub(crate) struct FileWatch;

impl ModuleFactory for FileWatch {
    fn create(&self) -> Result<Vec<Module>> {
        let watched_files = Config::watched_files()?;

        let mut modules = Vec::new();

        for file in watched_files {
            let body = fs_err::read_to_string(&file)?;

            let module = Module::new(file.to_string(), body, 3);

            modules.push(module);
        }

        Ok(modules)
    }
}

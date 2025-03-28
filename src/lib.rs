// #![warn(missing_docs)]
#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(unknown_lints)] // For nightly lints
#![allow(clippy::uninlined_format_args)]
#![allow(unstable_name_collisions)]

pub mod cli;

mod args;
mod config;
mod date_time;
mod fs;
mod last_updated;
mod system;
mod template;

pub(crate) use config::Config;
pub(crate) use date_time::DateTime;
pub(crate) use fs::FsTools;
pub(crate) use system::System;

#![warn(clippy::pedantic)]
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
mod model;
mod motd;
mod system;
mod template;

pub(crate) use config::Config;
pub(crate) use fs::{
    binary_exists_on_path, config_dir, drive_usage, modified_time,
};
pub(crate) use model::{
    DateTime, LastUpdated, MotdContext, MotdContextBuilder, Ram,
};
pub(crate) use system::System;

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

pub(crate) mod module;

mod config;
mod date_time;
mod error;
mod fs;
mod system;

pub(crate) use config::Config;
pub(crate) use date_time::DateTime;
pub(crate) use error::MarkotdError;
pub(crate) use fs::FsTools;
pub(crate) use system::System;

pub(crate) type Result<T> = std::result::Result<T, MarkotdError>;

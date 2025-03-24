use color_eyre::Result;
use color_eyre::eyre::eyre;
use derive_builder::Builder;
use minijinja::Environment;
use serde::Serialize;

use crate::DateTime;
use crate::fs::{Filesystem, FsMaxWidth};
use crate::last_updated::LastUpdated;

#[derive(Serialize)]
pub struct Ram {
    used: String,
    pct: String,
    total: String,
}

impl Ram {
    pub fn new(used: String, pct: String, total: String) -> Self {
        Self { used, pct, total }
    }
}

#[derive(Serialize, Builder)]
#[builder(pattern = "owned")]
pub struct MotdContext {
    distro: String,
    hostname: String,
    username: String,
    now: String,
    uptime: DateTime,
    last_updated: Option<LastUpdated>,
    ram: Ram,
    filesystems: Vec<Filesystem>,
    fs_max_width: FsMaxWidth,
}

impl MotdContext {
    pub fn render(&self, template: &str) -> Result<String> {
        let mut env = Environment::new();

        env.add_filter("repeat", repeat);
        env.add_filter("ljust", ljust);
        env.add_filter("rjust", rjust);

        let output = env.render_str(template, self)?;

        Ok(output)
    }
}

pub struct Template {
    pub body: &'static str,
    pub headings_in_width: bool,
}

pub fn get_template(name: &str) -> Result<Template> {
    match name.to_lowercase().as_str() {
        "json" => {
            Ok(Template {
                body: include_str!("../templates/motd.json"),
                headings_in_width: false,
            })
        },
        "md" => {
            Ok(Template {
                body: include_str!("../templates/motd.md"),
                headings_in_width: true,
            })
        },
        other => Err(eyre!("Unknown template: '{other}'")),
    }
}

fn repeat(value: &str, amount: usize) -> String {
    value.repeat(amount)
}

fn ljust(value: &str, width: usize) -> String {
    // let fill_char = fill_char.unwrap_or(" ");

    format!("{value:<width$}")
}

fn rjust(value: &str, width: usize) -> String {
    format!("{value:>width$}")
}

use color_eyre::Result;
use minijinja::Environment;
use serde::Serialize;

use crate::fs::Filesystem;

#[derive(Serialize)]
pub struct Uptime {
    time: String,
    date: String,
}

impl Uptime {
    pub fn new(time: String, date: String) -> Self {
        Self { time, date }
    }
}

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

#[derive(Serialize)]
pub struct MotdContext {
    distro: String,
    hostname: String,
    username: String,
    now: String,
    uptime: Uptime,
    ram: Ram,
    drives: Vec<Filesystem>,
}

impl MotdContext {
    pub fn new(
        distro: String,
        hostname: String,
        username: String,
        now: String,
        uptime: Uptime,
        ram: Ram,
        drives: Vec<Filesystem>,
    ) -> Self {
        Self { distro, hostname, username, now, uptime, ram, drives }
    }
}

const JSON_TEMPLATE: &str = include_str!("../templates/motd.json");
const MD_TEMPLATE: &str = include_str!("../templates/motd.md");

pub fn get_environment() -> Result<Environment<'static>> {
    let mut env = Environment::new();
    env.add_template("json", JSON_TEMPLATE)?;
    env.add_template("md", MD_TEMPLATE)?;

    Ok(env)
}

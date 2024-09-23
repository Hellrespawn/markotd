use minijinja::Environment;
use color_eyre::Result;
use serde::Serialize;

#[derive(Serialize)]
struct Uptime<'u> {
    time: &'u str,
    date: &'u str,
}
#[derive(Serialize)]

struct Ram<'r> {
    used: &'r str,
    pct: &'r str,
    total: &'r str,
}
#[derive(Serialize)]

struct Drive<'d> {
    fs: &'d str,
    size: &'d str,
    used: &'d str,
    pct: &'d str,
    avail: &'d str,
    target: &'d str,
}

#[derive(Serialize)]
pub struct MotdContext<'mt> {
    distro: &'mt str,
    hostname: &'mt str,
    username: &'mt str,
    uptime: &'mt Uptime<'mt>,
    ram: &'mt Ram<'mt>,
    drives: &'mt [Drive<'mt>],
}

impl<'mt> MotdContext<'mt> {
    pub fn test() -> Self {
        Self {
            distro: "Distro Linux",
            hostname: "HostName",
            username: "UserName",
            uptime: &Uptime { time: "Uptime.Time", date: "Uptime.Date" },
            ram: &Ram { used: "Ram.Used", pct: "Ram.Pct", total: "Ram.Total" },
            drives: &[Drive {
                fs: "/dev/sda1",
                size: "1.23T",
                used: "403G",
                pct: "12.34",
                avail: "796G",
                target: "/boot",
            }],
        }
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

use color_eyre::Result;

use crate::{
    template::{get_environment, MotdContext, Ram, Uptime},
    DateTime, FsTools, System,
};
// use itertools::Itertools;

// use crate::fs::FsTools;
// use crate::module::get_module_factories;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = FsTools::config()?.join("markotd.conf");

    if config.is_file() {
        dotenvy::from_path(config)?;
    }

    let env = get_environment()?;

    let template = env.get_template("md")?;

    let distro = System::platform_name();
    let hostname = System::hostname()?;
    let username = System::username();

    let now = DateTime::format_date(DateTime::now());

    let uptime = Uptime::new(System::uptime()?, System::boot_time()?);

    let ram = System::memory_usage()?;

    let drives = FsTools::drive_usage()?;

    let context =
        MotdContext::new(distro, hostname, username, now, uptime, ram, drives);

    let out = template.render(context)?;

    println!("{out}");

    Ok(())



    // let modules = get_module_factories()
    //     .into_iter()
    //     .map(|f| f.create())
    //     .collect::<Result<Vec<_>>>()?;

    // let iter = modules.into_iter().flatten();

    // print!(
    //     "{}",
    //     iter.map(|module| module.to_string())
    //         .intersperse("\n".to_owned())
    //         .collect::<String>()
    // );

    // Ok(())
}

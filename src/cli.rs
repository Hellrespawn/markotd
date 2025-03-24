use crate::args::Args;
use crate::fs::FsMaxWidth;
use crate::last_updated::get_last_update_time;
use crate::template::{MotdContextBuilder, get_template};
use crate::{DateTime, FsTools, System};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let config = FsTools::config()?.join("markotd.conf");

    if config.is_file() {
        dotenvy::from_path_override(config)?;
    }

    let template = get_template(&args.template)?;

    let distro = System::platform_name();
    let hostname = System::hostname()?;
    let username = System::username();

    let now = DateTime::format_date(DateTime::now());

    let uptime = DateTime::new(System::uptime()?, System::boot_time()?);

    let ram = System::memory_usage()?;

    let filesystems = FsTools::drive_usage()?;
    let fs_max_width =
        FsMaxWidth::from_filesystems(&filesystems, template.headings_in_width);

    let last_update_time = get_last_update_time()?;

    let builder = MotdContextBuilder::default();

    let context = builder
        .distro(distro)
        .hostname(hostname)
        .username(username)
        .now(now)
        .uptime(uptime)
        .ram(ram)
        .filesystems(filesystems)
        .fs_max_width(fs_max_width)
        .last_updated(last_update_time)
        .build()?;

    let out = context.render(template.body)?;

    println!("{out}");

    Ok(())
}

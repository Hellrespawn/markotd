use chrono::Local;
use filetime::FileTime;
use itertools::Itertools;

use crate::fs::FsTools;
use crate::module::get_module_factories;
use crate::Config;

pub fn main() {
    let path = FsTools::tmp().join(".markotd");
    let now = Local::now();

    if let Ok(last_run) = FsTools::get_last_update_time(&path) {
        let duration = now.naive_local() - last_run;

        if duration < Config::show_every_hours() {
            return;
        }
    }

    print!(
        "{}",
        get_module_factories()
            .iter()
            .filter_map(|f| f.create().map(|m| m.to_string()))
            .intersperse("\n".to_owned())
            .collect::<String>()
    );

    FsTools::touch(&path);

    filetime::set_file_mtime(
        &path,
        FileTime::from_unix_time(now.timestamp(), 0),
    )
    .expect("Unable to update file mtime.");
}

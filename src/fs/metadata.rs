use camino::Utf8Path;
use chrono::{DateTime, Local, NaiveDateTime};
use color_eyre::Result;
use color_eyre::eyre::eyre;

pub(crate) fn modified_time(path: &Utf8Path) -> Result<NaiveDateTime> {
    if !path.is_file() {
        return Err(eyre!("Path not found: {}", path));
    }

    let metadata = path.metadata()?;
    let mtime = metadata.modified()?;
    let datetime: DateTime<Local> = mtime.into();

    Ok(datetime.naive_local())
}

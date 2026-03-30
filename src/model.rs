use derive_builder::Builder;
use serde::Serialize;

use crate::fs::{Filesystem, FsMaxWidth};

#[derive(Serialize)]
pub(crate) struct Ram {
    used: String,
    pct: String,
    total: String,
}

impl Ram {
    pub(crate) fn new(used: String, pct: String, total: String) -> Self {
        Self { used, pct, total }
    }
}

#[derive(Serialize)]
pub(crate) struct DateTime {
    time_since: String,
    date: String,
}

impl DateTime {
    pub(crate) fn new(time_since: String, date: String) -> Self {
        Self { time_since, date }
    }
}

#[derive(Serialize)]
pub(crate) struct LastUpdated {
    time_since: String,
    date: String,
    app: String,
}

impl LastUpdated {
    pub(crate) fn new(time_since: String, date: String, app: String) -> Self {
        Self { time_since, date, app }
    }
}

#[derive(Serialize, Builder)]
#[builder(pattern = "owned")]
pub(crate) struct MotdContext {
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

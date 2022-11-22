use crate::Misc;
use systemstat::{saturating_sub_bytes, Filesystem as SystemStatFilesystem};

use super::FsMaxLength;

pub(crate) struct Filesystem {
    pub(crate) fs: String,
    pub(crate) size: String,
    pub(crate) used: String,
    pub(crate) avail: String,
    pub(crate) pct: String,
    pub(crate) mount: String,
}

impl Filesystem {
    pub(crate) fn from_system_stat_filesystem(
        filesystem: SystemStatFilesystem,
    ) -> Self {
        let used = saturating_sub_bytes(filesystem.total, filesystem.avail);
        Filesystem {
            fs: Filesystem::format_fs(filesystem.fs_mounted_from),
            size: filesystem.total.to_string(),
            used: used.to_string(),
            avail: filesystem.avail.to_string(),
            pct: format!(
                "{:05.2}%",
                Misc::pct_from_byte_sizes(used, filesystem.total)
            ),
            mount: filesystem.fs_mounted_on,
        }
    }

    pub(crate) fn headings() -> Self {
        Filesystem {
            fs: "filesystem".to_owned(),
            size: "size".to_owned(),
            used: "used".to_owned(),
            avail: "avail".to_owned(),
            pct: "pct".to_owned(),
            mount: "mount".to_owned(),
        }
    }

    pub(crate) fn to_aligned_string(&self, max_lengths: FsMaxLength) -> String {
        let FsMaxLength {
            fs,
            mount,
            size,
            used,
            avail,
            pct,
        } = max_lengths;

        format!(
            "| {:>fs$} | {:size$} | {:used$} | {:avail$} | {:pct$} | {:mount$} | ",
            self.fs, self.size, self.used, self.avail, self.pct, self.mount
        )
    }

    fn format_fs(fs: String) -> String {
        if fs.starts_with(char::is_alphabetic) {
            fs[..3].to_owned()
        } else {
            fs
        }
    }
}

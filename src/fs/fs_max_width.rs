use std::cmp::max;

use serde::Serialize;

use super::Filesystem;

#[derive(Default, Copy, Clone, Serialize)]
pub(crate) struct FsMaxWidth {
    pub(crate) fs: usize,
    pub(crate) size: usize,
    pub(crate) used: usize,
    pub(crate) avail: usize,
    pub(crate) pct: usize,
    pub(crate) target: usize,
}

impl FsMaxWidth {
    pub(crate) fn from_filesystems(
        filesystems: &[Filesystem],
        include_headers: bool,
    ) -> Self {
        let mut fs_lengths = FsMaxWidth::default();

        for fs in filesystems {
            fs_lengths.fs = max(fs_lengths.fs, fs.fs.len());
            fs_lengths.size = max(fs_lengths.size, fs.size.len());
            fs_lengths.used = max(fs_lengths.used, fs.used.len());
            fs_lengths.avail = max(fs_lengths.avail, fs.avail.len());
            fs_lengths.pct = max(fs_lengths.pct, fs.pct.len());
            fs_lengths.target = max(fs_lengths.target, fs.target.len());
        }

        if include_headers {
            fs_lengths.fs = max(fs_lengths.fs, "filesystem".len());
            fs_lengths.size = max(fs_lengths.size, "size".len());
            fs_lengths.used = max(fs_lengths.used, "used".len());
            fs_lengths.avail = max(fs_lengths.avail, "avail".len());
            fs_lengths.pct = max(fs_lengths.pct, "pct".len());
            fs_lengths.target = max(fs_lengths.target, "target".len());
        }

        fs_lengths
    }
}

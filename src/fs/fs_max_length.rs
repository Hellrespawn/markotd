use super::Filesystem;
use std::cmp::max;

#[derive(Default, Copy, Clone)]
pub(crate) struct FsMaxLength {
    pub(crate) fs: usize,
    pub(crate) size: usize,
    pub(crate) used: usize,
    pub(crate) avail: usize,
    pub(crate) pct: usize,
    pub(crate) mount: usize,
}

impl FsMaxLength {
    pub(crate) fn from_filesystems(filesystems: &[Filesystem]) -> Self {
        let mut fs_lengths = FsMaxLength::default();

        for fs in filesystems {
            fs_lengths.fs = max(fs_lengths.fs, fs.fs.len());
            fs_lengths.size = max(fs_lengths.size, fs.size.len());
            fs_lengths.used = max(fs_lengths.used, fs.used.len());
            fs_lengths.avail = max(fs_lengths.avail, fs.avail.len());
            fs_lengths.pct = max(fs_lengths.pct, fs.pct.len());
            fs_lengths.mount = max(fs_lengths.mount, fs.mount.len());
        }

        fs_lengths
    }
}

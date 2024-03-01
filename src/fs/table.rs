use itertools::Itertools;

use super::{Filesystem, FsMaxLength};

pub(crate) struct FilesystemTable {
    filesystems: Vec<Filesystem>,
}

impl FilesystemTable {
    pub(crate) fn new(mut filesystems: Vec<Filesystem>) -> Self {
        filesystems.insert(0, Filesystem::headings());

        Self { filesystems }
    }

    fn table_separator(max_lengths: FsMaxLength) -> String {
        let fill_char = "-";

        format!(
            "| {} | {} | {} | {} | {} | {} |",
            fill_char.repeat(max_lengths.fs),
            fill_char.repeat(max_lengths.size),
            fill_char.repeat(max_lengths.used),
            fill_char.repeat(max_lengths.avail),
            fill_char.repeat(max_lengths.pct),
            fill_char.repeat(max_lengths.mount)
        )
    }
}

impl std::fmt::Display for FilesystemTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_lengths = FsMaxLength::from_filesystems(&self.filesystems);

        let mut iter =
            self.filesystems.iter().map(|fs| fs.to_aligned_string(max_lengths));

        let headings =
            iter.next().expect("Unable to get headings from iterator,");

        let separator = FilesystemTable::table_separator(max_lengths);

        let drives = iter.intersperse("\n".to_owned()).collect::<String>();

        write!(f, "{}\n{}\n{}", headings, separator, drives)
    }
}

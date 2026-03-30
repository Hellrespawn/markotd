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

const COLUMNS: [FilesystemColumn; 6] = [
    FilesystemColumn::Fs,
    FilesystemColumn::Size,
    FilesystemColumn::Used,
    FilesystemColumn::Avail,
    FilesystemColumn::Pct,
    FilesystemColumn::Target,
];

impl FsMaxWidth {
    pub(crate) fn from_filesystems(
        filesystems: &[Filesystem],
        include_headers: bool,
    ) -> Self {
        let mut widths = FsMaxWidth::default();

        for fs in filesystems {
            for column in COLUMNS {
                column.update_width(&mut widths, column.value(fs).len());
            }
        }

        if include_headers {
            for column in COLUMNS {
                column.update_width(&mut widths, column.header().len());
            }
        }

        widths
    }
}

#[derive(Copy, Clone)]
enum FilesystemColumn {
    Fs,
    Size,
    Used,
    Avail,
    Pct,
    Target,
}

impl FilesystemColumn {
    fn header(self) -> &'static str {
        match self {
            Self::Fs => "filesystem",
            Self::Size => "size",
            Self::Used => "used",
            Self::Avail => "avail",
            Self::Pct => "pct",
            Self::Target => "target",
        }
    }

    fn value(self, filesystem: &Filesystem) -> &str {
        match self {
            Self::Fs => &filesystem.fs,
            Self::Size => &filesystem.size,
            Self::Used => &filesystem.used,
            Self::Avail => &filesystem.avail,
            Self::Pct => &filesystem.pct,
            Self::Target => &filesystem.target,
        }
    }

    fn update_width(self, widths: &mut FsMaxWidth, candidate: usize) {
        let current = match self {
            Self::Fs => &mut widths.fs,
            Self::Size => &mut widths.size,
            Self::Used => &mut widths.used,
            Self::Avail => &mut widths.avail,
            Self::Pct => &mut widths.pct,
            Self::Target => &mut widths.target,
        };

        *current = max(*current, candidate);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fs_max_width_uses_longest_values_without_headers() {
        let filesystems = vec![
            Filesystem {
                fs: "short".to_owned(),
                size: "1T".to_owned(),
                used: "20G".to_owned(),
                avail: "900G".to_owned(),
                pct: "9%".to_owned(),
                target: "/mnt/a".to_owned(),
            },
            Filesystem {
                fs: "/dev/very-long-device-name".to_owned(),
                size: "999T".to_owned(),
                used: "222G".to_owned(),
                avail: "1000G".to_owned(),
                pct: "100%".to_owned(),
                target: "/mnt/very/long/path".to_owned(),
            },
        ];

        let widths = FsMaxWidth::from_filesystems(&filesystems, false);

        assert_eq!(widths.fs, "/dev/very-long-device-name".len());
        assert_eq!(widths.size, "999T".len());
        assert_eq!(widths.used, "222G".len());
        assert_eq!(widths.avail, "1000G".len());
        assert_eq!(widths.pct, "100%".len());
        assert_eq!(widths.target, "/mnt/very/long/path".len());
    }

    #[test]
    fn test_fs_max_width_includes_headers_when_requested() {
        let filesystems = vec![Filesystem {
            fs: "fs".to_owned(),
            size: "1".to_owned(),
            used: "2".to_owned(),
            avail: "3".to_owned(),
            pct: "4".to_owned(),
            target: "5".to_owned(),
        }];

        let widths = FsMaxWidth::from_filesystems(&filesystems, true);

        assert_eq!(widths.fs, "filesystem".len());
        assert_eq!(widths.size, "size".len());
        assert_eq!(widths.used, "used".len());
        assert_eq!(widths.avail, "avail".len());
        assert_eq!(widths.pct, "pct".len());
        assert_eq!(widths.target, "target".len());
    }
}

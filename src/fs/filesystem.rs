use super::FsMaxLength;
use once_cell::sync::Lazy;
use regex::Regex;

static FS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([[:alpha:]]:|/dev)").expect("Unable to compile regex.")
});

pub(crate) struct Filesystem {
    pub(crate) fs: String,
    pub(crate) size: String,
    pub(crate) used: String,
    pub(crate) avail: String,
    pub(crate) pct: String,
    pub(crate) mount: String,
}

impl Filesystem {
    pub(crate) fn from_df_line(line: &str) -> Option<Self> {
        let segments = line
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();

        // df prints mountpoints without quotes, so it can be multiple segments.

        assert!(
            segments.len() >= 6,
            "df -Ph did not return the the expected amount of six columns:\n{:#?}",
            segments
        );

        let mut iter = segments.into_iter();

        let fs = Filesystem {
            fs: iter.next().unwrap(),
            size: iter.next().unwrap(),
            used: iter.next().unwrap(),
            avail: iter.next().unwrap(),
            pct: iter.next().unwrap(),
            mount: iter.collect::<Vec<_>>().join(" "),
        };

        if Self::filter_filesystem(&fs) {
            Some(fs)
        } else {
            None
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
            "| {:fs$} | {:>size$} | {:>used$} | {:>avail$} | {:>pct$} | {:mount$} | ",
            self.fs, self.size, self.used, self.avail, self.pct, self.mount
        )
    }

    fn filter_filesystem(filesystem: &Filesystem) -> bool {
        FS_REGEX.is_match(&filesystem.fs)
            && !filesystem.mount.contains("docker")
    }
}

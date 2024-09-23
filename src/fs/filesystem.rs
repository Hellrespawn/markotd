use color_eyre::eyre::eyre;
use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

use super::FsMaxLength;

static FS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([[:alpha:]]:|/dev)").expect("Unable to compile regex.")
});

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Filesystem {
    pub(crate) fs: String,
    pub(crate) size: String,
    pub(crate) used: String,
    pub(crate) avail: String,
    pub(crate) pct: String,
    pub(crate) target: String,
}

impl Filesystem {
    pub(crate) fn from_df_line(line: &str) -> Result<Option<Self>> {
        let segments = line
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();

        // df prints mountpoints without quotes, so it can be multiple segments.
        if segments.len() < 6 {
            return Err(eyre!("df -Ph did not return the the expected amount of six columns:\n{:#?}",
            segments));
        }

        let pct_index = segments.iter().position(|s| s.ends_with('%'));

        if pct_index.is_none() {
            return Err(eyre!("Unable to determine percentage column of df."));
        }

        let pct_index = pct_index.unwrap();

        let pct = segments[pct_index].trim_end_matches('%').to_owned();
        let avail = segments[pct_index - 1].clone();
        let used = segments[pct_index - 2].clone();
        let size = segments[pct_index - 3].clone();

        let fs = segments[0..pct_index - 3].join(" ").clone();
        let target = segments[pct_index + 1..segments.len()].join(" ").clone();

        let fs = Filesystem { fs, size, used, avail, pct, target };

        if Self::filter_filesystem(&fs) {
            Ok(Some(fs))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn headings() -> Self {
        Filesystem {
            fs: "filesystem".to_owned(),
            size: "size".to_owned(),
            used: "used".to_owned(),
            avail: "avail".to_owned(),
            pct: "pct".to_owned(),
            target: "target".to_owned(),
        }
    }

    pub(crate) fn to_aligned_string(&self, max_lengths: FsMaxLength) -> String {
        let FsMaxLength { fs, mount, size, used, avail, pct } = max_lengths;

        format!(
            "| {:fs$} | {:>size$} | {:>used$} | {:>avail$} | {:>pct$} | {:mount$} | ",
            self.fs, self.size, self.used, self.avail, self.pct, self.target
        )
    }

    fn filter_filesystem(filesystem: &Filesystem) -> bool {
        FS_REGEX.is_match(&filesystem.fs)
            && !filesystem.target.contains("docker")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filesystem_both_spaces() -> Result<()> {
        let df_line = "C:\\Program Files\\Docker\\Docker\\resources  1.9T  234G  1.6T  13% /Docker/host name/thing";

        let fs = Filesystem::from_df_line(df_line)?;

        let reference = Filesystem {
            fs: "C:\\Program Files\\Docker\\Docker\\resources".to_owned(),
            size: "1.9T".to_owned(),
            used: "234G".to_owned(),
            avail: "1.6T".to_owned(),
            pct: "13%".to_owned(),
            target: "/Docker/host name/thing".to_owned(),
        };

        assert_eq!(fs, Some(reference));

        Ok(())
    }

    #[test]
    fn test_filesystem_fs_space() -> Result<()> {
        let df_line = "C:\\Program Files\\Docker\\Docker\\resources  1.9T  234G  1.6T  13% /Docker/host";

        let fs = Filesystem::from_df_line(df_line)?;

        let reference = Filesystem {
            fs: "C:\\Program Files\\Docker\\Docker\\resources".to_owned(),
            size: "1.9T".to_owned(),
            used: "234G".to_owned(),
            avail: "1.6T".to_owned(),
            pct: "13%".to_owned(),
            target: "/Docker/host".to_owned(),
        };

        assert_eq!(fs, Some(reference));

        Ok(())
    }

    #[test]
    fn test_filesystem_mount_spaces() -> Result<()> {
        let df_line = "C:\\Docker\\Docker\\resources  1.9T  234G  1.6T  13% /Docker/host name/thing";

        let fs = Filesystem::from_df_line(df_line)?;

        let reference = Filesystem {
            fs: "C:\\Docker\\Docker\\resources".to_owned(),
            size: "1.9T".to_owned(),
            used: "234G".to_owned(),
            avail: "1.6T".to_owned(),
            pct: "13%".to_owned(),
            target: "/Docker/host name/thing".to_owned(),
        };

        assert_eq!(fs, Some(reference));

        Ok(())
    }

    #[test]
    fn test_filesystem_no_spaces() -> Result<()> {
        let df_line =
            "C:\\Docker\\Docker\\resources  1.9T  234G  1.6T  13% /Docker/host";

        let fs = Filesystem::from_df_line(df_line)?;

        let reference = Filesystem {
            fs: "C:\\Docker\\Docker\\resources".to_owned(),
            size: "1.9T".to_owned(),
            used: "234G".to_owned(),
            avail: "1.6T".to_owned(),
            pct: "13%".to_owned(),
            target: "/Docker/host".to_owned(),
        };

        assert_eq!(fs, Some(reference));

        Ok(())
    }
}

use color_eyre::Result;
use color_eyre::eyre::eyre;
use regex::Regex;
use serde::Serialize;

use crate::Config;

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
    pub(crate) fn from_df_line(line: &str) -> Result<Self> {
        let segments = line
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();

        if segments.len() < 6 {
            return Err(eyre!(
                "df -Ph did not return the the expected amount of six columns:\n{:#?}",
                segments
            ));
        }

        let pct_index =
            segments.iter().position(|s| s.ends_with('%')).ok_or_else(
                || eyre!("Unable to determine percentage column of df."),
            )?;

        let pct = segments[pct_index].clone();
        let avail = segments[pct_index - 1].clone();
        let used = segments[pct_index - 2].clone();
        let size = segments[pct_index - 3].clone();

        let fs = segments[0..pct_index - 3].join(" ").clone();
        let target = segments[pct_index + 1..segments.len()].join(" ").clone();

        let fs = Filesystem { fs, size, used, avail, pct, target };

        Ok(fs)
    }

    pub(super) fn filter_filesystem(&self, filter: &FilesystemFilter) -> bool {
        let is_whitelisted = filter
            .whitelist
            .iter()
            .any(|re| re.is_match(&self.fs) || re.is_match(&self.target));

        let is_blacklisted = filter
            .blacklist
            .iter()
            .any(|re| re.is_match(&self.fs) || re.is_match(&self.target));

        is_whitelisted && !is_blacklisted
    }
}

pub(super) struct FilesystemFilter {
    whitelist: Vec<Regex>,
    blacklist: Vec<Regex>,
}

impl FilesystemFilter {
    pub(super) fn from_config(config: &Config) -> Result<Self> {
        let mut whitelist =
            vec![Regex::new(r"^[[:alpha:]]:")?, Regex::new("^/dev")?];

        if let Some(regex) = config.df_whitelist_regex() {
            whitelist.push(Regex::new(regex)?);
        }

        let mut blacklist =
            vec![Regex::new(r"(?i)docker")?, Regex::new(r"^none$")?];

        if let Some(regex) = config.df_blacklist_regex() {
            blacklist.push(Regex::new(regex)?);
        }

        Ok(Self { whitelist, blacklist })
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

        assert_eq!(fs, reference);

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

        assert_eq!(fs, reference);

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

        assert_eq!(fs, reference);

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

        assert_eq!(fs, reference);

        Ok(())
    }

    #[test]
    fn test_filesystem_docker_filtered_by_default() {
        let filter = FilesystemFilter::from_config(&Config::default()).unwrap();
        let filesystem = Filesystem {
            fs: "C:\\Docker\\Docker\\resources".to_owned(),
            size: "1.9T".to_owned(),
            used: "234G".to_owned(),
            avail: "1.6T".to_owned(),
            pct: "13%".to_owned(),
            target: "/Docker/host".to_owned(),
        };

        assert!(!filesystem.filter_filesystem(&filter));
    }

    #[test]
    fn test_filesystem_none_filtered_by_default() {
        let filter = FilesystemFilter::from_config(&Config::default()).unwrap();
        let filesystem = Filesystem {
            fs: "none".to_owned(),
            size: "3.9G".to_owned(),
            used: "0".to_owned(),
            avail: "3.9G".to_owned(),
            pct: "0%".to_owned(),
            target: "/dev/tty".to_owned(),
        };

        assert!(!filesystem.filter_filesystem(&filter));
    }
}

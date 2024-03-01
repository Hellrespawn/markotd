use color_eyre::eyre::eyre;
use color_eyre::Result;
use itertools::Itertools;

use super::{Module, ModuleFactory};
use crate::fs::FsTools;

#[derive(Debug, PartialEq)]
struct Link<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> Link<'a> {
    fn new(name: &'a str, url: &'a str) -> Self {
        Self { name, url }
    }

    fn from_string(string: &'a str) -> Result<Self> {
        let (name, url) = string
            .split_once(':')
            .ok_or(eyre!("'{}' is not a valid link!", string))?;

        Ok(Link::new(name, url))
    }
}

pub(crate) struct Links;

impl ModuleFactory for Links {
    fn create(&self) -> Result<Vec<Module>> {
        let path = FsTools::home()?.join(".markotd-links");

        if let Ok(file_contents) = fs_err::read_to_string(path) {
            let links = Self::parse_file_contents(&file_contents)?;

            if links.is_empty() {
                return Ok(vec![]);
            }

            let module = Module::new(
                "Useful Links".to_owned(),
                Self::format_links(&links),
                2,
            );

            Ok(vec![module])
        } else {
            Ok(vec![])
        }
    }
}

impl Links {
    fn format_links(links: &[Link]) -> String {
        let max_name_length = Self::get_max_name_length(links);

        links
            .iter()
            .map(|link| Self::format_link(link, max_name_length))
            .intersperse("\n".to_owned())
            .collect()
    }

    fn format_link(link: &Link, max_name_length: usize) -> String {
        let pad = " ".repeat(max_name_length - link.name.len());
        format!("- {}[{}]({})", pad, link.name, link.url)
    }

    fn get_max_name_length(links: &[Link]) -> usize {
        links.iter().map(|link| link.name.len()).max().unwrap_or(0)
    }

    fn parse_file_contents(file_contents: &str) -> Result<Vec<Link>> {
        file_contents
            .trim()
            .lines()
            .map(str::trim)
            .filter(|l| !l.starts_with('#'))
            .map(Link::from_string)
            .collect::<Result<Vec<_>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "
    # Example:
    #
    Syncthing:https://sleipnir.no-ip.net/syncthing
    Gitea:https://sleipnir.no-ip.net/gitea";

    #[test]
    fn test_parse_file_contents() -> Result<()> {
        let input = TEST_DATA;

        let expected = vec![
            Link::new("Syncthing", "https://sleipnir.no-ip.net/syncthing"),
            Link::new("Gitea", "https://sleipnir.no-ip.net/gitea"),
        ];

        let links = Links::parse_file_contents(input)?;

        assert_eq!(links, expected);

        Ok(())
    }

    #[test]
    fn test_parse_file_contents_empty_file() -> Result<()> {
        let input = "\n#Comment goes here. ";

        let expected = Vec::new();

        let links = Links::parse_file_contents(input)?;

        assert_eq!(links, expected);

        Ok(())
    }

    #[test]
    fn test_parse_file_contents_invalid_link() {
        let input = "This does not have a colon.";

        assert!(Links::parse_file_contents(input).is_err());
    }

    #[test]
    fn test_get_max_name_length() -> Result<()> {
        let links = Links::parse_file_contents(TEST_DATA)?;

        let expected = 9;

        assert_eq!(Links::get_max_name_length(&links), expected);

        Ok(())
    }

    #[test]
    fn test_get_max_name_length_empty_vec() {
        assert_eq!(Links::get_max_name_length(&Vec::new()), 0);
    }

    #[test]
    fn test_format_link() -> Result<()> {
        let links = Links::parse_file_contents(TEST_DATA)?;

        let max_name_length = Links::get_max_name_length(&links);

        let expected = vec![
            "- [Syncthing](https://sleipnir.no-ip.net/syncthing)",
            "-     [Gitea](https://sleipnir.no-ip.net/gitea)",
        ];

        let actual = links
            .iter()
            .map(|link| Links::format_link(link, max_name_length))
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_format_links() -> Result<()> {
        let links = Links::parse_file_contents(TEST_DATA)?;

        let expected = "- [Syncthing](https://sleipnir.no-ip.net/syncthing)\n-     [Gitea](https://sleipnir.no-ip.net/gitea)";

        let actual = Links::format_links(&links);

        assert_eq!(expected, actual);

        Ok(())
    }
}

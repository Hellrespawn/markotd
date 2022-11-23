use super::{Module, ModuleFactory};
use crate::fs::FsTools;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Link<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> Link<'a> {
    fn new(name: &'a str, url: &'a str) -> Self {
        Self { name, url }
    }

    fn from_string(string: &'a str) -> Self {
        let (name, url) = string
            .split_once(':')
            .unwrap_or_else(|| panic!("'{}' is not a valid link!", string));

        Link::new(name, url)
    }
}

pub(crate) struct Links;

impl ModuleFactory for Links {
    fn create(&self) -> Option<Module> {
        let path = FsTools::home().join(".markotd-links");

        if let Ok(file_contents) = std::fs::read_to_string(&path) {
            let links: Vec<Link> = Self::parse_file_contents(&file_contents);

            if links.is_empty() {
                return None;
            }

            let module = Module::new(
                "Useful Links".to_owned(),
                Self::format_links(&links),
                2,
            );

            Some(module)
        } else {
            None
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
        links
            .iter()
            .map(|link| link.name.len())
            .max()
            .expect("Passed empty iterator to `get_max_name_length`")
    }

    fn parse_file_contents(file_contents: &str) -> Vec<Link> {
        file_contents
            .trim()
            .lines()
            .map(str::trim)
            .filter(|l| !l.starts_with('#'))
            .map(Link::from_string)
            .collect::<Vec<_>>()
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
    fn test_parse_file_contents() {
        let input = TEST_DATA;

        let expected = vec![
            Link::new("Syncthing", "https://sleipnir.no-ip.net/syncthing"),
            Link::new("Gitea", "https://sleipnir.no-ip.net/gitea"),
        ];

        let links = Links::parse_file_contents(input);

        assert_eq!(links, expected);
    }

    #[test]
    fn test_parse_file_contents_empty_file() {
        let input = "\n#Comment goes here. ";

        let expected = Vec::new();

        let links = Links::parse_file_contents(input);

        assert_eq!(links, expected);
    }

    #[test]
    #[should_panic]
    fn test_parse_file_contents_invalid_link() {
        let input = "This does not have a colon.";

        Links::parse_file_contents(input);
    }

    #[test]
    fn test_get_max_name_length() {
        let links = Links::parse_file_contents(TEST_DATA);

        let expected = 9;

        assert_eq!(Links::get_max_name_length(&links), expected);
    }

    #[test]
    #[should_panic]
    fn test_get_max_name_length_empty_vec() {
        Links::get_max_name_length(&Vec::new());
    }

    #[test]
    fn test_format_link() {
        let links = Links::parse_file_contents(TEST_DATA);

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
    }

    #[test]
    fn test_format_links() {
        let links = Links::parse_file_contents(TEST_DATA);

        let expected = "- [Syncthing](https://sleipnir.no-ip.net/syncthing)\n-     [Gitea](https://sleipnir.no-ip.net/gitea)";

        let actual = Links::format_links(&links);

        assert_eq!(expected, actual);
    }
}

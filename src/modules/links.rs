use super::{Module, ModuleFactory};
use itertools::Itertools;
use std::path::Path;

type Link<'a> = (&'a str, &'a str);

pub(crate) struct Links;

impl ModuleFactory for Links {
    fn create(&self) -> Option<Module> {
        if let Some(file_contents) =
            Links::get_file_contents(&"~/markotd-links")
        {
            let links: Vec<Link> = Links::parse_file_contents(&file_contents);

            if links.is_empty() {
                return None;
            }

            let body = Links::format_links(&links);

            let module = Module::new("Useful Links".to_owned(), body, 2);

            Some(module)
        } else {
            None
        }
    }
}

impl Links {
    fn format_links(links: &[(&str, &str)]) -> String {
        let max_name_length = Links::get_max_name_length(links);

        links
            .iter()
            .map(|(n, l)| Links::format_link(n, l, max_name_length))
            .intersperse("\n".to_owned())
            .collect()
    }

    fn format_link(name: &str, link: &str, max_name_length: usize) -> String {
        let pad = " ".repeat(max_name_length - name.len());
        format!(" - {}[{}]({})", pad, name, link)
    }

    fn get_max_name_length(links: &[Link]) -> usize {
        links
            .iter()
            .map(|(n, _)| n.len())
            .max()
            .expect("Passed empty iterator to `get_max_name_length`")
    }

    fn parse_file_contents(file_contents: &str) -> Vec<Link> {
        file_contents
            .trim()
            .lines()
            .map(str::trim)
            .filter(|l| !l.starts_with('#'))
            .map(|l| {
                l.split_once(':')
                    .unwrap_or_else(|| panic!("'{}' is not a valid link!", l))
            })
            .collect::<Vec<_>>()
    }

    fn get_file_contents<P>(path: &P) -> Option<String>
    where
        P: AsRef<Path>,
    {
        std::fs::read_to_string(path).ok()
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
            ("Syncthing", "https://sleipnir.no-ip.net/syncthing"),
            ("Gitea", "https://sleipnir.no-ip.net/gitea"),
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
            " - [Syncthing](https://sleipnir.no-ip.net/syncthing)",
            " -     [Gitea](https://sleipnir.no-ip.net/gitea)",
        ];

        let actual = links
            .iter()
            .map(|(name, link)| Links::format_link(name, link, max_name_length))
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_format_links() {
        let links = Links::parse_file_contents(TEST_DATA);

        let expected = " - [Syncthing](https://sleipnir.no-ip.net/syncthing)\n -     [Gitea](https://sleipnir.no-ip.net/gitea)";

        let actual = Links::format_links(&links);

        assert_eq!(expected, actual);
    }
}

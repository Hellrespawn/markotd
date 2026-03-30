use color_eyre::Result;
use color_eyre::eyre::eyre;
use minijinja::{Environment, Error, ErrorKind};
use serde_json::to_string;

use crate::MotdContext;

pub struct Template {
    pub body: &'static str,
    pub headings_in_width: bool,
}

pub fn get_template(name: &str) -> Result<Template> {
    match name.to_lowercase().as_str() {
        "json" => {
            Ok(Template {
                body: include_str!("../templates/motd.json.tmpl"),
                headings_in_width: false,
            })
        },
        "md" => {
            Ok(Template {
                body: include_str!("../templates/motd.md.tmpl"),
                headings_in_width: true,
            })
        },
        "toml" => {
            Ok(Template {
                body: include_str!("../templates/motd.toml.tmpl"),
                headings_in_width: false,
            })
        },
        other => Err(eyre!("Unknown template: '{other}'")),
    }
}

pub(crate) fn render(context: &MotdContext, template: &str) -> Result<String> {
    let mut env = Environment::new();

    env.add_filter("repeat", repeat);
    env.add_filter("ljust", ljust);
    env.add_filter("rjust", rjust);
    env.add_filter("json_string", json_string);
    env.add_filter("color", color);
    env.add_filter("bold", bold);
    env.add_filter("dim", dim);
    env.add_filter("italic", italic);
    env.add_filter("underline", underline);

    let output = env.render_str(template, context)?;

    Ok(output)
}

fn repeat(value: &str, amount: usize) -> String {
    value.repeat(amount)
}

fn ljust(value: &str, width: usize) -> String {
    format!("{value:<width$}")
}

fn rjust(value: &str, width: usize) -> String {
    format!("{value:>width$}")
}

fn json_string(value: &str) -> Result<String, minijinja::Error> {
    to_string(value)
        .map_err(|err| Error::new(ErrorKind::BadSerialization, err.to_string()))
}

fn color(value: &str, name: &str) -> Result<String, minijinja::Error> {
    let code = match name {
        "black" => "30",
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "gray" | "grey" => "90",
        "bright_red" => "91",
        "bright_green" => "92",
        "bright_yellow" => "93",
        "bright_blue" => "94",
        "bright_magenta" => "95",
        "bright_cyan" => "96",
        "bright_white" => "97",
        other => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("unknown color '{other}'"),
            ));
        },
    };

    Ok(ansi(value, code))
}

fn bold(value: &str) -> String {
    ansi(value, "1")
}

fn dim(value: &str) -> String {
    ansi(value, "2")
}

fn italic(value: &str) -> String {
    ansi(value, "3")
}

fn underline(value: &str) -> String {
    ansi(value, "4")
}

fn ansi(value: &str, code: &str) -> String {
    format!("\u{1b}[{code}m{value}\u{1b}[0m")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bold_wraps_with_ansi_sequence() {
        assert_eq!(bold("markotd"), "\u{1b}[1mmarkotd\u{1b}[0m");
    }

    #[test]
    fn test_color_uses_requested_ansi_sequence() {
        let output = color("markotd", "cyan").unwrap();

        assert_eq!(output, "\u{1b}[36mmarkotd\u{1b}[0m");
    }

    #[test]
    fn test_color_rejects_unknown_color() {
        let err = color("markotd", "orange").unwrap_err();

        assert_eq!(err.kind(), ErrorKind::InvalidOperation);
        assert!(err.to_string().contains("unknown color 'orange'"));
    }
}

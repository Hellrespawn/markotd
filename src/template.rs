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
                body: include_str!("../templates/motd.json"),
                headings_in_width: false,
            })
        },
        "md" => {
            Ok(Template {
                body: include_str!("../templates/motd.md"),
                headings_in_width: true,
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

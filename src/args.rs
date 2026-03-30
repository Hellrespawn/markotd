use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "md", value_parser = ["json", "md", "toml"])]
    pub template: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_args_default_template_is_markdown() {
        let args = Args::parse_from(["markotd"]);

        assert_eq!(args.template, "md");
    }
}

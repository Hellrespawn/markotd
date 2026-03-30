use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "json", value_parser = ["json", "md", "toml"])]
    pub template: String,
}

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value = "json")]
    pub template: String,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

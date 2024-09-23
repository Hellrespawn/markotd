use color_eyre::Result;

use crate::template::{get_environment, MotdContext};
// use itertools::Itertools;

// use crate::fs::FsTools;
// use crate::module::get_module_factories;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let env = get_environment()?;

    let context = MotdContext::test();

    let template = env.get_template("md")?;

    let out = template.render(context)?;

    println!("{out}");

    Ok(())


    // let env = FsTools::config()?.join("markotd.conf");

    // if env.is_file() {
    //     dotenvy::from_path(env)?;
    // }

    // let modules = get_module_factories()
    //     .into_iter()
    //     .map(|f| f.create())
    //     .collect::<Result<Vec<_>>>()?;

    // let iter = modules.into_iter().flatten();

    // print!(
    //     "{}",
    //     iter.map(|module| module.to_string())
    //         .intersperse("\n".to_owned())
    //         .collect::<String>()
    // );

    // Ok(())
}

use crate::modules::get_module_factories;
use itertools::Itertools;

fn wrapper() -> Result<String, Box<dyn std::error::Error>> {
    let modules = get_module_factories();

    let output = modules
        .iter()
        .filter_map(|f| f().map(|m| m.to_string()))
        .intersperse("\n".to_owned())
        .collect();

    Ok(output)
}

pub fn main() {
    match wrapper() {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error),
    }
}

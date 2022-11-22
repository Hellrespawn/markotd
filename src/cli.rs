use itertools::Itertools;

use crate::modules::MODULE_FACTORIES;

fn wrapper() -> Result<String, Box<dyn std::error::Error>> {
    let output = MODULE_FACTORIES
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

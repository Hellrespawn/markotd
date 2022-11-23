use itertools::Itertools;

use crate::modules::get_module_factories;

pub fn main() {
    let output: String = get_module_factories()
        .iter()
        .filter_map(|f| f.create().map(|m| m.to_string()))
        .intersperse("\n".to_owned())
        .collect();

    print!("{}", output);
}

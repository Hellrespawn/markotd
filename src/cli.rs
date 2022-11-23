use itertools::Itertools;

use crate::module::get_module_factories;

pub fn main() {
    print!(
        "{}",
        get_module_factories()
            .iter()
            .filter_map(|f| f.create().map(|m| m.to_string()))
            .intersperse("\n".to_owned())
            .collect::<String>()
    );
}

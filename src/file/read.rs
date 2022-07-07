use std::fs;

use serde_json::Value;

pub fn read_data() -> String {
    let filepath = super::common::expand_tilde("~/config/koe/data.json".to_string());

    fs::read_to_string(&filepath).expect("Something went wrong reading the file")
}

pub fn read_nas() -> String {
    let nas_file = super::nas::get_file();
    fs::read_to_string(&nas_file).expect("Something went wrong reading the file")
}

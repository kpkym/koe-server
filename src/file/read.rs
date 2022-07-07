use std::fs;

use serde_json::Value;

pub fn read_data() -> String {
    let filepath = format!("{}/config/koe/data.json", std::env::var("HOME").unwrap());

    fs::read_to_string(&filepath).expect("Something went wrong reading the file")
}

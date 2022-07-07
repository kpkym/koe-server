use std::fs;

pub fn read_file() -> String {
    let filepath = format!("{}/config/koe/data.json", std::env::var("HOME").unwrap());
    fs::read_to_string(&filepath).expect("Something went wrong reading the file")
}

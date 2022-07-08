use std::process::Command;

use serde_json::Value;

use crate::file::read;

pub fn koe_open() {}

pub fn koe_list(code: String) -> String {
    let mut result = vec![];
    let v: Value = serde_json::from_str(&read::read_nas()).expect("err");

    for x in v.as_array().unwrap().into_iter() {
        if x["type"].as_str().unwrap() == "D" && x.to_string().contains(code.as_str()) {
            result.push(x);
        }
    }

    let search_list = crate::file::search::search(code);
    for x in search_list.iter() {
        result.push(x);
    }

    serde_json::to_string(&result).unwrap()
}

pub fn direct_open(path: String) {
    Command::new("open")
        .arg(path)
        .output()
        .unwrap();
}

pub fn get_data(value: Vec<String>) -> String {
    let mut result = vec![];
    let v: Value = serde_json::from_str(&read::read_data()).expect("err");

    for x in v.as_array().unwrap().into_iter() {
        if value.contains(&x["code"].as_str().unwrap().to_string()) {
            result.push(x);
        }
    }

    serde_json::to_string(&result).unwrap()
}


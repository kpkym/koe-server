use serde_json::Value;

use crate::file::read;

pub fn koe_open() {}

pub fn koe_list(code: String) -> String {

    "".to_string()
}

pub fn direct_open() {}

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


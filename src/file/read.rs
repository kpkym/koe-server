use std::fs;

use serde_json::Value;

pub fn read_data(value: &Vec<String>) -> String {
    let filepath = format!("{}/config/koe/data.json", std::env::var("HOME").unwrap());
    let content = fs::read_to_string(&filepath).expect("Something went wrong reading the file");

    let v: Value = serde_json::from_str(&content).expect("err");

    let mut result = vec![];

    for x in v.as_array().unwrap().into_iter() {
        if value.contains(&x["code"].as_str().unwrap().to_string()) {
            result.push(x);
        }
    }


    println!("{}", v.as_array().unwrap().len());
    println!("{:?}", result);

    serde_json::to_string(&result).unwrap()
}

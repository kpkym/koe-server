use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

use serde_json::{Map, Value};

pub fn search(code: String) -> Vec<Value> {
    let mut result = vec![];

    for x in vec![
        super::common::expand_tilde("~/Downloads".to_string()),
        super::common::expand_tilde("~/ooo/koe".to_string()),
    ] {
        let output = Command::new("fd")
            .arg(code.as_str())
            .arg(x)
            .output()
            .unwrap();

        let string = String::from_utf8(output.stdout).unwrap();
        if string != "" {
            for x in string.split("\n").collect::<Vec<&str>>() {
                if x != "" {
                    let mut v = Map::new();

                    v.insert("type".to_string(), Value::String("D".to_string()));
                    v.insert("path".to_string(), Value::String(x.to_string()));
                    result.push(Value::Object(v));
                }
            }
        }
    }

    result
}

use std::io::{self, Write};
use std::process::Command;

pub fn search(code: String) -> Vec<String> {
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
                    result.push(x.to_string());
                }
            }
        }
    }

    result
}

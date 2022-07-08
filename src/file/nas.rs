use std::ffi::OsStr;
use std::fs;

pub fn get_file() -> String {
    let dir = super::common::expand_tilde("~/config/koe/nas_koe_data".to_string());
    let paths = fs::read_dir(dir).unwrap();
    let mut result = vec![];


    for path in paths {
        let path = path.unwrap().path();
        if let Some("json") = path.extension().and_then(OsStr::to_str) {
            result.push(path);
        }
    }

    result.sort();
    result.last().unwrap().to_str().unwrap().to_string()
}

use serde_json::Value;

use crate::file::read;

pub fn koe_open() {}

pub fn koe_list() {}

pub fn direct_open() {}

pub fn get_data(value: &Vec<String>) -> String {
    read::read_data(value)
}


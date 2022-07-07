use serde_json::Value;

pub fn get_type(str: &String) -> Value {
    serde_json::from_str(str.as_str()).expect("err")
}

pub fn list_code(v: &Value) -> Vec<String> {
    let code = v.as_array().unwrap()
        .into_iter()
        .map(|i| i.as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    // vec![code.to_string()]
    code
}

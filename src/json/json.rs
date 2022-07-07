use serde_json::Value;

pub fn get_type(str: &String) -> Value {
    serde_json::from_str(str.as_str()).expect("err")
}

pub fn list_code(v: &Value) -> Vec<String> {
    let code = v.as_array()
        .and_then(|value| value.get(0))
        .and_then(|value| value.as_str())
        .unwrap();

    vec![code.to_string()]
}

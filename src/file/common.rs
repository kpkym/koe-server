pub fn expand_tilde(str: String) -> String {
    if str.chars().next().unwrap() != '~' {
        return str;
    }

    let string = str.chars().skip(1).take(str.len() - 1).collect::<String>();
    format!("{}{}", std::env::var("HOME").unwrap(), string)
}

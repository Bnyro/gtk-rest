pub fn format_json_string(text: String) -> String {
    let object: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(text.as_str());
    if !object.is_ok() {
        return text;
    }
    let formatted = serde_json::to_string_pretty(&object.unwrap());
    match formatted {
        Ok(formatted) => return formatted,
        _ => return text,
    };
}

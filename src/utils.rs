pub fn format_json_string(text: String) -> String {
    let object: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
    let formatted = serde_json::to_string_pretty(&object);
    match formatted {
        Ok(formatted) => return formatted,
        _ => return text,
    };
}

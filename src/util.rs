use std::io::Write;

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
    std::io::stdout().flush().unwrap();
}

pub fn get_str_or_default(json: &serde_json::Value, key: &str, default: &str) -> String {
    json.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

pub fn parse_title(title: &str) -> (u16, String) {
    let parts: Vec<&str> = title.split(": ").collect();
    let number = parts[0].trim_start_matches('#').parse::<u16>().unwrap_or(0);
    (number, parts[1].trim_end_matches(" (Albumversie)").to_string())
}

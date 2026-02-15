pub fn escape_dsl_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn format_identifier(name: &str) -> String {
    let normalized = name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
    normalized
        .chars()
        .next()
        .map(|first| {
            if first.is_ascii_alphabetic() || first == '_' {
                format!("{}{}", first, &normalized[1..])
            } else {
                format!("_{}", normalized)
            }
        })
        .unwrap_or_else(|| "element".to_string())
}

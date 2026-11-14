/// Writes Rust literals that compile whatever the user typed.
pub struct RustLiteral;

impl RustLiteral {
    pub fn string(text: &str) -> String {
        let mut literal = String::with_capacity(text.len() + 2);
        literal.push('"');
        for character in text.chars() {
            match character {
                '"' => literal.push_str("\\\""),
                '\\' => literal.push_str("\\\\"),
                '\n' => literal.push_str("\\n"),
                '\r' => literal.push_str("\\r"),
                '\t' => literal.push_str("\\t"),
                other => literal.push(other),
            }
        }
        literal.push('"');
        literal
    }

    pub fn pixels(value: i64) -> String {
        format!("{value}.0")
    }

    pub fn string_list(items: &[String]) -> String {
        let mut parts = Vec::with_capacity(items.len());
        for item in items {
            parts.push(Self::string(item));
        }
        format!("vec![{}]", parts.join(", "))
    }
}

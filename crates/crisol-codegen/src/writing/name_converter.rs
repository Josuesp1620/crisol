/// Converts between the naming styles generated code uses.
pub struct NameConverter;

impl NameConverter {
    /// `CustomerForm` becomes `customer_form`.
    pub fn snake_case(pascal: &str) -> String {
        let mut result = String::with_capacity(pascal.len() + 4);
        for (position, character) in pascal.chars().enumerate() {
            if character.is_ascii_uppercase() {
                if position > 0 {
                    result.push('_');
                }
                result.push(character.to_ascii_lowercase());
            } else {
                result.push(character);
            }
        }
        result
    }

    /// `selection_changed` becomes `SelectionChanged`.
    pub fn pascal_case(snake: &str) -> String {
        let mut result = String::with_capacity(snake.len());
        let mut upper_next = true;
        for character in snake.chars() {
            if character == '_' {
                upper_next = true;
            } else if upper_next {
                result.push(character.to_ascii_uppercase());
                upper_next = false;
            } else {
                result.push(character);
            }
        }
        result
    }
}

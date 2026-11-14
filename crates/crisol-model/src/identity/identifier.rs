use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{ModelError, ModelResult};

/// A `snake_case` name that is also a valid Rust identifier: control names (`btn_save`)
/// and event handler methods (`save`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Identifier(String);

impl Identifier {
    const RESERVED_WORDS: [&'static str; 38] = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
        "type", "unsafe", "use", "where", "while", "gen",
    ];

    pub fn parse(text: &str) -> ModelResult<Self> {
        if Self::is_valid(text) {
            Ok(Self(text.to_owned()))
        } else {
            Err(ModelError::InvalidIdentifier(text.to_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(text: &str) -> bool {
        let mut characters = text.chars();
        let Some(first) = characters.next() else {
            return false;
        };
        if !first.is_ascii_lowercase() || Self::RESERVED_WORDS.contains(&text) {
            return false;
        }
        for character in characters {
            if !(character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_') {
                return false;
            }
        }
        true
    }
}

impl TryFrom<String> for Identifier {
    type Error = ModelError;

    fn try_from(text: String) -> ModelResult<Self> {
        Self::parse(&text)
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

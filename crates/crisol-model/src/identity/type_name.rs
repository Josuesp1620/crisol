use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{ModelError, ModelResult};

/// A `PascalCase` name that becomes a Rust type: the form (`CustomerForm`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TypeName(String);

impl TypeName {
    pub fn parse(text: &str) -> ModelResult<Self> {
        if Self::is_valid(text) {
            Ok(Self(text.to_owned()))
        } else {
            Err(ModelError::InvalidTypeName(text.to_owned()))
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
        if !first.is_ascii_uppercase() {
            return false;
        }
        for character in characters {
            if !character.is_ascii_alphanumeric() {
                return false;
            }
        }
        true
    }
}

impl TryFrom<String> for TypeName {
    type Error = ModelError;

    fn try_from(text: String) -> ModelResult<Self> {
        Self::parse(&text)
    }
}

impl From<TypeName> for String {
    fn from(type_name: TypeName) -> Self {
        type_name.0
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

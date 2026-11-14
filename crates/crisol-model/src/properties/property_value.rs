use serde::{Deserialize, Serialize};

/// The value of one property, written in the file as a plain JSON value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Boolean(bool),
    Integer(i64),
    Number(f64),
    Text(String),
    List(Vec<String>),
}

impl PropertyValue {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            PropertyValue::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            PropertyValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            PropertyValue::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            PropertyValue::List(items) => Some(items),
            _ => None,
        }
    }
}

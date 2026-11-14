use crisol_model::properties::PropertyValue;

use super::{PropertyCategory, PropertyType};

/// One property of a control, as the property grid and the generators see it.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertySchema {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub category: PropertyCategory,
    pub value_type: PropertyType,
    pub default_value: PropertyValue,
}

impl PropertySchema {
    pub fn text(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        category: PropertyCategory,
        default_text: &str,
    ) -> Self {
        Self {
            name,
            label,
            description,
            category,
            value_type: PropertyType::Text,
            default_value: PropertyValue::Text(default_text.to_owned()),
        }
    }

    pub fn boolean(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        category: PropertyCategory,
        default_flag: bool,
    ) -> Self {
        Self {
            name,
            label,
            description,
            category,
            value_type: PropertyType::Boolean,
            default_value: PropertyValue::Boolean(default_flag),
        }
    }

    pub fn integer(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        category: PropertyCategory,
        range: (i64, i64),
        default_number: i64,
    ) -> Self {
        Self {
            name,
            label,
            description,
            category,
            value_type: PropertyType::Integer {
                minimum: range.0,
                maximum: range.1,
            },
            default_value: PropertyValue::Integer(default_number),
        }
    }

    pub fn choice(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        category: PropertyCategory,
        options: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            label,
            description,
            category,
            value_type: PropertyType::Choice(options),
            default_value: PropertyValue::Text(options[0].to_owned()),
        }
    }

    pub fn text_list(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        category: PropertyCategory,
    ) -> Self {
        Self {
            name,
            label,
            description,
            category,
            value_type: PropertyType::TextList,
            default_value: PropertyValue::List(Vec::new()),
        }
    }
}

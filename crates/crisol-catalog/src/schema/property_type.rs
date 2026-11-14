use crisol_model::properties::PropertyValue;

/// The kind of value a property accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Text,
    Boolean,
    Integer { minimum: i64, maximum: i64 },
    Choice(&'static [&'static str]),
    TextList,
}

impl PropertyType {
    pub fn accepts(&self, value: &PropertyValue) -> bool {
        match (self, value) {
            (PropertyType::Text, PropertyValue::Text(_))
            | (PropertyType::Boolean, PropertyValue::Boolean(_))
            | (PropertyType::TextList, PropertyValue::List(_)) => true,
            (PropertyType::Integer { minimum, maximum }, PropertyValue::Integer(number)) => {
                (*minimum..=*maximum).contains(number)
            }
            (PropertyType::Choice(options), PropertyValue::Text(text)) => {
                options.contains(&text.as_str())
            }
            _ => false,
        }
    }
}

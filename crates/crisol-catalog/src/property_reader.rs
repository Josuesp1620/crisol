use crate::schema::WidgetSchema;
use crisol_model::document::Node;
use crisol_model::properties::PropertyValue;

/// Reads a node's properties, falling back to the catalog defaults.
pub struct PropertyReader<'context> {
    node: &'context Node,
    schema: Option<&'context WidgetSchema>,
}

impl<'context> PropertyReader<'context> {
    pub fn new(node: &'context Node, schema: Option<&'context WidgetSchema>) -> Self {
        Self { node, schema }
    }

    pub fn text(&self, name: &str) -> String {
        match self.value(name) {
            Some(PropertyValue::Text(text)) => text.clone(),
            _ => String::new(),
        }
    }

    pub fn boolean(&self, name: &str) -> bool {
        matches!(self.value(name), Some(PropertyValue::Boolean(true)))
    }

    pub fn integer(&self, name: &str) -> i64 {
        match self.value(name) {
            Some(PropertyValue::Integer(number)) => *number,
            _ => 0,
        }
    }

    pub fn list(&self, name: &str) -> Vec<String> {
        match self.value(name) {
            Some(PropertyValue::List(items)) => items.clone(),
            _ => Vec::new(),
        }
    }

    fn value(&self, name: &str) -> Option<&PropertyValue> {
        if let Some(stored) = self.node.properties.get(name) {
            return Some(stored);
        }
        let schema = self.schema?;
        schema
            .property(name)
            .map(|property| &property.default_value)
    }
}

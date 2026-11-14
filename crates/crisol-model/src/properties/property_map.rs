use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::PropertyValue;

/// Properties that differ from the control's defaults, sorted by name so the file is canonical.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyMap(BTreeMap<String, PropertyValue>);

impl PropertyMap {
    pub fn get(&self, name: &str) -> Option<&PropertyValue> {
        self.0.get(name)
    }

    /// Sets a value and returns the previous one, if any.
    pub fn set(&mut self, name: &str, value: PropertyValue) -> Option<PropertyValue> {
        self.0.insert(name.to_owned(), value)
    }

    /// Removes a value so the control falls back to its default.
    pub fn reset(&mut self, name: &str) -> Option<PropertyValue> {
        self.0.remove(name)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &PropertyValue)> {
        self.0.iter()
    }
}

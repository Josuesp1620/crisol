use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::identity::Identifier;

/// Event name (`click`) to the handler method that answers it (`save`), sorted by event.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventMap(BTreeMap<String, Identifier>);

impl EventMap {
    pub fn handler_for(&self, event: &str) -> Option<&Identifier> {
        self.0.get(event)
    }

    /// Binds an event and returns the handler it replaced, if any.
    pub fn bind(&mut self, event: &str, handler: Identifier) -> Option<Identifier> {
        self.0.insert(event.to_owned(), handler)
    }

    pub fn unbind(&mut self, event: &str) -> Option<Identifier> {
        self.0.remove(event)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn bindings(&self) -> impl Iterator<Item = (&String, &Identifier)> {
        self.0.iter()
    }
}

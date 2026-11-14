use std::fmt;

use serde::{Deserialize, Serialize};

/// The catalog name of a control (`Button`, `TextBox`). The catalog knows what each kind means.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WidgetKind(String);

impl WidgetKind {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WidgetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

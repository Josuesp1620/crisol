use crate::workspace::PropertyKey;

/// One line of the property grid, ready to draw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyRow {
    pub key: PropertyKey,
    pub category: String,
    pub label: String,
    pub value: String,
    pub description: String,
    /// Differs from the default, so it is stored in the file and shown in bold.
    pub modified: bool,
    /// An event row: the value is a method name.
    pub is_event: bool,
    pub read_only: bool,
}

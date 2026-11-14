use crate::Command;

/// One applied step: what the user did and how to take it back.
#[derive(Debug)]
pub struct HistoryEntry {
    pub label: String,
    pub(crate) reverse: Box<dyn Command>,
}

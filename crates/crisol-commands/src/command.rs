use std::fmt::Debug;

use crisol_model::document::FormDocument;

use crate::CommandResult;

/// A change to a document that knows how to undo itself.
pub trait Command: Debug {
    /// Stable identifier shared with the command line and the assistant (`node.insert`).
    fn id(&self) -> &'static str;

    /// What the history list shows (`Add btn_save`).
    fn label(&self) -> String;

    /// Applies the change and returns the command that reverts it.
    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>>;
}

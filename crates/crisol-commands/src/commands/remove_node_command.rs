use crisol_model::document::FormDocument;
use crisol_model::identity::NodeId;

use super::RestoreSubtreeCommand;
use crate::{Command, CommandResult};

/// Deletes a control and everything inside it.
#[derive(Debug, Clone)]
pub struct RemoveNodeCommand {
    pub node: NodeId,
}

impl Command for RemoveNodeCommand {
    fn id(&self) -> &'static str {
        "node.remove"
    }

    fn label(&self) -> String {
        "Delete control".to_owned()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let subtree = document.detach(self.node)?;
        Ok(Box::new(RestoreSubtreeCommand { subtree }))
    }
}

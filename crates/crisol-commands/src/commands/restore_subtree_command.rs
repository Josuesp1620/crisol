use crisol_model::document::{DetachedSubtree, FormDocument};

use super::RemoveNodeCommand;
use crate::{Command, CommandResult};

/// Puts a deleted control back exactly where it was.
#[derive(Debug, Clone)]
pub struct RestoreSubtreeCommand {
    pub subtree: DetachedSubtree,
}

impl Command for RestoreSubtreeCommand {
    fn id(&self) -> &'static str {
        "node.restore"
    }

    fn label(&self) -> String {
        "Restore control".to_owned()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        document.reattach(self.subtree.clone())?;
        Ok(Box::new(RemoveNodeCommand {
            node: self.subtree.root,
        }))
    }
}

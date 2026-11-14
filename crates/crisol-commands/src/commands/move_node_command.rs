use crisol_model::document::FormDocument;
use crisol_model::identity::NodeId;

use crate::{Command, CommandResult};

/// Moves a control to another position or container.
#[derive(Debug, Clone)]
pub struct MoveNodeCommand {
    pub node: NodeId,
    pub new_parent: NodeId,
    pub index: usize,
}

impl Command for MoveNodeCommand {
    fn id(&self) -> &'static str {
        "node.move"
    }

    fn label(&self) -> String {
        "Move control".to_owned()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let (old_parent, old_index) = document.move_node(self.node, self.new_parent, self.index)?;
        Ok(Box::new(MoveNodeCommand {
            node: self.node,
            new_parent: old_parent,
            index: old_index,
        }))
    }
}

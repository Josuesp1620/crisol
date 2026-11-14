use crisol_model::document::{FormDocument, Node};
use crisol_model::identity::NodeId;

use super::RemoveNodeCommand;
use crate::{Command, CommandResult};

/// Adds a new control as a child of a container.
#[derive(Debug, Clone)]
pub struct InsertNodeCommand {
    pub parent: NodeId,
    pub index: usize,
    pub node: Node,
}

impl Command for InsertNodeCommand {
    fn id(&self) -> &'static str {
        "node.insert"
    }

    fn label(&self) -> String {
        format!("Add {}", self.node.name)
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        document.insert(self.parent, self.index, self.node.clone())?;
        Ok(Box::new(RemoveNodeCommand { node: self.node.id }))
    }
}

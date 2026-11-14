use crisol_model::ModelError;
use crisol_model::document::FormDocument;
use crisol_model::identity::{Identifier, NodeId};

use crate::{Command, CommandResult};

/// Gives a control a new name, which must stay unique in the form.
#[derive(Debug, Clone)]
pub struct RenameNodeCommand {
    pub node: NodeId,
    pub name: Identifier,
}

impl Command for RenameNodeCommand {
    fn id(&self) -> &'static str {
        "node.rename"
    }

    fn label(&self) -> String {
        format!("Rename to {}", self.name)
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        if let Some(owner) = document.find_by_name(self.name.as_str())
            && owner != self.node
        {
            return Err(ModelError::DuplicateName(self.name.to_string()).into());
        }
        let node = document.node_mut(self.node)?;
        let previous = std::mem::replace(&mut node.name, self.name.clone());
        Ok(Box::new(RenameNodeCommand {
            node: self.node,
            name: previous,
        }))
    }
}

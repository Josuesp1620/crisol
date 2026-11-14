use crisol_model::document::FormDocument;
use crisol_model::identity::NodeId;
use crisol_model::layout::Placement;

use crate::{Command, CommandResult};

/// Changes where a control sits in its container: position, size, cell or anchors.
#[derive(Debug, Clone)]
pub struct SetPlacementCommand {
    pub node: NodeId,
    pub placement: Placement,
}

impl Command for SetPlacementCommand {
    fn id(&self) -> &'static str {
        "node.place"
    }

    fn label(&self) -> String {
        "Change layout".to_owned()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let node = document.node_mut(self.node)?;
        let previous = std::mem::replace(&mut node.placement, self.placement.clone());
        Ok(Box::new(SetPlacementCommand {
            node: self.node,
            placement: previous,
        }))
    }
}

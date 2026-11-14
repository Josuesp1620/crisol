use crisol_model::identity::NodeId;

/// The controls selected in the designer. The first one is the primary selection, the one
/// the property grid shows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SelectionSet {
    nodes: Vec<NodeId>,
}

impl SelectionSet {
    pub fn select_only(&mut self, node: NodeId) {
        self.nodes.clear();
        self.nodes.push(node);
    }

    /// Adds the node, or removes it when it was already selected (Ctrl+click).
    pub fn toggle(&mut self, node: NodeId) {
        if let Some(position) = self.nodes.iter().position(|selected| *selected == node) {
            self.nodes.remove(position);
        } else {
            self.nodes.push(node);
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn primary(&self) -> Option<NodeId> {
        self.nodes.first().copied()
    }

    pub fn contains(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    pub fn nodes(&self) -> &[NodeId] {
        &self.nodes
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Drops nodes that no longer exist, after an undo or a delete.
    pub fn retain_existing(&mut self, exists: impl Fn(NodeId) -> bool) {
        self.nodes.retain(|node| exists(*node));
    }
}

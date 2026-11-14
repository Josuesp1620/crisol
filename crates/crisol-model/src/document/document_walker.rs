use crate::identity::NodeId;

use super::FormDocument;

/// Visits the nodes of a document in reading order: parent first, then children in order.
pub struct DocumentWalker<'document> {
    document: &'document FormDocument,
}

impl<'document> DocumentWalker<'document> {
    pub fn new(document: &'document FormDocument) -> Self {
        Self { document }
    }

    /// Every node of the form, the root first.
    pub fn all(&self) -> Vec<NodeId> {
        self.subtree(self.document.root_id())
    }

    /// The node and all its descendants, the node first.
    pub fn subtree(&self, start: NodeId) -> Vec<NodeId> {
        let mut visited = Vec::new();
        let mut pending = vec![start];
        while let Some(current) = pending.pop() {
            visited.push(current);
            let children = self.document.children_of(current);
            for child in children.iter().rev() {
                pending.push(*child);
            }
        }
        visited
    }

    /// Whether `candidate` is `ancestor` itself or lies somewhere below it.
    pub fn is_within(&self, candidate: NodeId, ancestor: NodeId) -> bool {
        let mut current = Some(candidate);
        while let Some(node) = current {
            if node == ancestor {
                return true;
            }
            current = self.document.parent_of(node);
        }
        false
    }
}

use crate::identity::NodeId;

use super::Node;

/// A node and all its descendants taken out of a document, with where they were,
/// so the removal can be undone exactly.
#[derive(Debug, Clone, PartialEq)]
pub struct DetachedSubtree {
    pub root: NodeId,
    pub parent: NodeId,
    pub index: usize,
    pub nodes: Vec<Node>,
}

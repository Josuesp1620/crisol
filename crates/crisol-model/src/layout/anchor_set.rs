use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::Edge;

/// The container edges a control keeps its distance to when the container resizes,
/// as in Windows Forms. Only used by absolute placement, which is desktop only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnchorSet(BTreeSet<Edge>);

impl AnchorSet {
    pub fn with_edges(edges: &[Edge]) -> Self {
        let mut set = BTreeSet::new();
        for edge in edges {
            set.insert(*edge);
        }
        Self(set)
    }

    pub fn contains(&self, edge: Edge) -> bool {
        self.0.contains(&edge)
    }

    pub fn toggle(&mut self, edge: Edge) {
        if !self.0.remove(&edge) {
            self.0.insert(edge);
        }
    }
}

impl Default for AnchorSet {
    fn default() -> Self {
        Self::with_edges(&[Edge::Top, Edge::Left])
    }
}

use crisol_model::document::FormDocument;
use crisol_model::geometry::Point;
use crisol_model::identity::NodeId;

use super::LayoutSnapshot;

/// Finds the controls under a point, the innermost first.
pub struct HitTester<'context> {
    snapshot: &'context LayoutSnapshot,
    document: &'context FormDocument,
}

impl<'context> HitTester<'context> {
    pub fn new(snapshot: &'context LayoutSnapshot, document: &'context FormDocument) -> Self {
        Self { snapshot, document }
    }

    /// Every node under the point except the form's root, the deepest in the tree first, so
    /// a control wins over the container around it. Alt+click walks this list to reach
    /// containers and controls hidden under others.
    pub fn stack_at(&self, point: Point) -> Vec<NodeId> {
        let mut hits: Vec<(usize, usize, NodeId)> = Vec::new();
        for (order, node) in self.snapshot.paint_order().iter().enumerate() {
            if *node == self.document.root_id() || !self.document.contains(*node) {
                continue;
            }
            if let Some(rect) = self.snapshot.bounds_of(*node)
                && rect.contains(point)
            {
                hits.push((self.depth_of(*node), order, *node));
            }
        }
        hits.sort_by(|first, second| second.0.cmp(&first.0).then(second.1.cmp(&first.1)));
        let mut stack = Vec::with_capacity(hits.len());
        for (_, _, node) in hits {
            stack.push(node);
        }
        stack
    }

    pub fn topmost_at(&self, point: Point) -> Option<NodeId> {
        self.stack_at(point).first().copied()
    }

    fn depth_of(&self, node: NodeId) -> usize {
        let mut depth = 0;
        let mut current = self.document.parent_of(node);
        while let Some(parent) = current {
            depth += 1;
            current = self.document.parent_of(parent);
        }
        depth
    }
}

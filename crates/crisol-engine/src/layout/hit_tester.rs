use crisol_model::document::FormDocument;
use crisol_model::geometry::Point;
use crisol_model::identity::NodeId;

use super::LayoutSnapshot;

/// Finds the controls under a point, the topmost first.
pub struct HitTester<'context> {
    snapshot: &'context LayoutSnapshot,
    document: &'context FormDocument,
}

impl<'context> HitTester<'context> {
    pub fn new(snapshot: &'context LayoutSnapshot, document: &'context FormDocument) -> Self {
        Self { snapshot, document }
    }

    /// Every node under the point except the form's root, the topmost first. Alt+click walks
    /// this list to reach controls hidden under others.
    pub fn stack_at(&self, point: Point) -> Vec<NodeId> {
        let mut hits = Vec::new();
        for node in self.snapshot.paint_order().iter().rev() {
            if *node == self.document.root_id() || !self.document.contains(*node) {
                continue;
            }
            if let Some(rect) = self.snapshot.bounds_of(*node)
                && rect.contains(point)
            {
                hits.push(*node);
            }
        }
        hits
    }

    pub fn topmost_at(&self, point: Point) -> Option<NodeId> {
        self.stack_at(point).first().copied()
    }
}

use std::collections::HashMap;

use crisol_model::geometry::Rect;
use crisol_model::identity::NodeId;

/// The bounds of every drawn node in canvas coordinates, in paint order (parents first).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutSnapshot {
    bounds: HashMap<NodeId, Rect>,
    paint_order: Vec<NodeId>,
}

impl LayoutSnapshot {
    pub fn record(&mut self, node: NodeId, rect: Rect) {
        if self.bounds.insert(node, rect).is_none() {
            self.paint_order.push(node);
        }
    }

    pub fn clear(&mut self) {
        self.bounds.clear();
        self.paint_order.clear();
    }

    pub fn bounds_of(&self, node: NodeId) -> Option<Rect> {
        self.bounds.get(&node).copied()
    }

    pub fn paint_order(&self) -> &[NodeId] {
        &self.paint_order
    }
}

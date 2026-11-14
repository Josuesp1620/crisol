use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;

/// Where a dragged or new control would land if released now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropTarget {
    pub container: NodeId,
    /// Position among the container's children.
    pub index: usize,
    /// The pointer relative to the container, for absolute panels.
    pub local_point: Point,
    /// A thin bar showing the insertion point in stacks and grids.
    pub marker: Option<Rect>,
}

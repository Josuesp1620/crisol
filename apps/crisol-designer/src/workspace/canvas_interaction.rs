use crisol_engine::dragging::{DragKind, DragSession, DropTarget};
use crisol_engine::snapping::SnapResult;
use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;

/// What the pointer is doing on the canvas.
#[derive(Debug, Clone, Default)]
pub enum CanvasInteraction {
    #[default]
    Idle,
    /// The button is down on a control but has not moved enough to start a drag.
    Pressing {
        node: NodeId,
        kind: DragKind,
        start: Point,
        rect: Rect,
    },
    /// A move or resize in progress; only the preview changes until release.
    Dragging {
        session: DragSession,
        preview: SnapResult,
        drop: Option<DropTarget>,
    },
}

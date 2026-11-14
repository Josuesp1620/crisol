use crisol_commands::Command;
use crisol_commands::commands::SetPlacementCommand;
use crisol_model::document::FormDocument;
use crisol_model::geometry::{Point, Rect, Size};
use crisol_model::identity::NodeId;
use crisol_model::layout::Placement;

use super::DragKind;
use crate::EngineSettings;
use crate::selection::ResizeHandle;
use crate::snapping::{SnapEngine, SnapResult};

/// One move or resize in progress. Until it finishes only the preview changes; the
/// document changes once, with one command, when the pointer is released.
#[derive(Debug, Clone)]
pub struct DragSession {
    pub node: NodeId,
    pub kind: DragKind,
    start_pointer: Point,
    start_rect: Rect,
    settings: EngineSettings,
}

impl DragSession {
    pub fn begin(
        node: NodeId,
        kind: DragKind,
        pointer: Point,
        rect: Rect,
        settings: EngineSettings,
    ) -> Self {
        Self {
            node,
            kind,
            start_pointer: pointer,
            start_rect: rect,
            settings,
        }
    }

    /// The rectangle the control would have with the pointer at `pointer`, snapped.
    pub fn preview(&self, pointer: Point, references: &[Rect]) -> SnapResult {
        let delta = self.start_pointer.distance_to(pointer);
        let engine = SnapEngine::new(self.settings.snap_distance);
        match self.kind {
            DragKind::Move => engine.snap_move(
                self.start_rect
                    .moved_to(self.start_rect.origin.offset_by(delta)),
                references,
            ),
            DragKind::Resize(handle) => {
                let resized = self.resized(delta, handle);
                let mut snapped = engine.snap_resize(resized, handle, references);
                snapped.rect = self.at_least_minimum(snapped.rect);
                snapped
            }
        }
    }

    pub fn has_moved(&self, pointer: Point) -> bool {
        self.start_pointer != pointer
    }

    /// The command that stores the final rectangle, for controls on an absolute panel.
    /// `parent_origin` converts canvas coordinates back into the panel's coordinates.
    pub fn finish(
        &self,
        final_rect: Rect,
        parent_origin: Point,
        document: &FormDocument,
    ) -> Option<Box<dyn Command>> {
        let node = document.node(self.node).ok()?;
        let Placement::Absolute { anchor, .. } = &node.placement else {
            return None;
        };
        let local = final_rect.moved_to(parent_origin.distance_to(final_rect.origin));
        Some(Box::new(SetPlacementCommand {
            node: self.node,
            placement: Placement::Absolute {
                rect: local,
                anchor: anchor.clone(),
            },
        }))
    }

    fn resized(&self, delta: Point, handle: ResizeHandle) -> Rect {
        let mut left = self.start_rect.left();
        let mut right = self.start_rect.right();
        let mut top = self.start_rect.top();
        let mut bottom = self.start_rect.bottom();
        if handle.moves_left_edge() {
            left += delta.x;
        }
        if handle.moves_right_edge() {
            right += delta.x;
        }
        if handle.moves_top_edge() {
            top += delta.y;
        }
        if handle.moves_bottom_edge() {
            bottom += delta.y;
        }
        Rect::new(left, top, right - left, bottom - top)
    }

    fn at_least_minimum(&self, rect: Rect) -> Rect {
        let minimum = self.settings.minimum_control_size;
        rect.resized_to(Size::new(
            rect.size.width.max(minimum),
            rect.size.height.max(minimum),
        ))
    }
}

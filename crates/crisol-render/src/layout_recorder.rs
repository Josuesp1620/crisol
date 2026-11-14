use std::cell::RefCell;
use std::rc::Rc;

use crisol_engine::layout::LayoutSnapshot;
use crisol_model::geometry::Rect;
use crisol_model::identity::NodeId;
use gpui::{AnyElement, Bounds, IntoElement, ParentElement, Pixels, Styled, canvas, div};

/// Collects where each control was drawn during a frame, shared with the editor.
#[derive(Clone, Default)]
pub struct LayoutRecorder {
    snapshot: Rc<RefCell<LayoutSnapshot>>,
}

impl LayoutRecorder {
    pub fn clear(&self) {
        self.snapshot.borrow_mut().clear();
    }

    pub fn snapshot(&self) -> LayoutSnapshot {
        self.snapshot.borrow().clone()
    }

    /// Wraps `element` so its bounds are recorded for `node` when it is laid out.
    pub fn measure(&self, node: NodeId, element: AnyElement) -> AnyElement {
        let snapshot = self.snapshot.clone();
        let probe = canvas(
            move |bounds, _window, _cx| snapshot.borrow_mut().record(node, Self::rect_of(bounds)),
            |_bounds, (), _window, _cx| {},
        )
        .absolute()
        .size_full();
        div()
            .relative()
            .flex()
            .flex_col()
            .child(element)
            .child(probe)
            .into_any_element()
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "window coordinates fit in i32"
    )]
    fn rect_of(bounds: Bounds<Pixels>) -> Rect {
        Rect::new(
            f32::from(bounds.origin.x).round() as i32,
            f32::from(bounds.origin.y).round() as i32,
            f32::from(bounds.size.width).round() as i32,
            f32::from(bounds.size.height).round() as i32,
        )
    }
}

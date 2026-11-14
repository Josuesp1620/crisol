use std::cell::RefCell;
use std::rc::Rc;

use crisol_engine::layout::LayoutSnapshot;
use crisol_model::geometry::Rect;
use crisol_model::identity::NodeId;
use gpui::{AnyElement, Bounds, IntoElement, ParentElement, Pixels, Styled, canvas, div};

/// Collects where each control was drawn during a frame, shared with the editor. While a
/// new frame is being laid out, readers get the last complete one.
#[derive(Clone, Default)]
pub struct LayoutRecorder {
    snapshot: Rc<RefCell<LayoutSnapshot>>,
    last_complete: Rc<RefCell<LayoutSnapshot>>,
}

impl LayoutRecorder {
    /// Starts recording a new frame, keeping the previous one for readers meanwhile.
    pub fn begin_frame(&self) {
        let finished = std::mem::take(&mut *self.snapshot.borrow_mut());
        if !finished.paint_order().is_empty() {
            *self.last_complete.borrow_mut() = finished;
        }
    }

    pub fn snapshot(&self) -> LayoutSnapshot {
        let current = self.snapshot.borrow();
        if current.paint_order().is_empty() {
            self.last_complete.borrow().clone()
        } else {
            current.clone()
        }
    }

    /// Wraps `element` so its bounds are recorded for `node` when it is laid out.
    pub fn measure(&self, node: NodeId, element: AnyElement) -> AnyElement {
        self.probe(node, element).into_any_element()
    }

    /// Like `measure`, but the element grows to fill its container: used for the form root,
    /// so it covers the whole designed window.
    pub fn measure_filling(&self, node: NodeId, element: AnyElement) -> AnyElement {
        self.probe(node, element).flex_1().into_any_element()
    }

    fn probe(&self, node: NodeId, element: AnyElement) -> gpui::Div {
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

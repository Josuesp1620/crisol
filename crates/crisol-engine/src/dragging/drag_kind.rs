use crate::selection::ResizeHandle;

/// What a drag on the canvas does to the selected control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragKind {
    Move,
    Resize(ResizeHandle),
}

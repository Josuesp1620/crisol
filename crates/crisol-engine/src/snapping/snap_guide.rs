use super::GuideAxis;

/// A line drawn while dragging to show what the control aligned with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapGuide {
    pub axis: GuideAxis,
    /// The x of a vertical guide or the y of a horizontal one.
    pub position: i32,
    /// Where the line starts and ends along its axis.
    pub start: i32,
    pub end: i32,
}

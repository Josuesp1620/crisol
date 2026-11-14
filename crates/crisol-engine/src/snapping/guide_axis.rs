/// Direction of a guide line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideAxis {
    /// A vertical line at an x position.
    Vertical,
    /// A horizontal line at a y position.
    Horizontal,
}

use crisol_model::geometry::Rect;

use super::SnapGuide;

/// The rectangle after snapping and the guides to draw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapResult {
    pub rect: Rect,
    pub guides: Vec<SnapGuide>,
}

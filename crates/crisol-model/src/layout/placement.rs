use serde::{Deserialize, Serialize};

use super::{AnchorSet, GridCell, SizeMode};
use crate::geometry::Rect;

/// Where a node sits inside its parent. The parent's kind decides which variant applies:
/// a stack places children in order, a grid in cells, a panel at absolute positions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    /// In the order of the parent's children; portable to every platform.
    Flow { width: SizeMode, height: SizeMode },
    /// In a grid cell; portable to every platform.
    Cell { cell: GridCell, width: SizeMode },
    /// At a fixed position with Windows Forms anchors; desktop only.
    Absolute { rect: Rect, anchor: AnchorSet },
}

impl Placement {
    pub fn flow() -> Self {
        Placement::Flow {
            width: SizeMode::Hug,
            height: SizeMode::Hug,
        }
    }

    pub fn is_portable(&self) -> bool {
        !matches!(self, Placement::Absolute { .. })
    }
}

impl Default for Placement {
    fn default() -> Self {
        Self::flow()
    }
}

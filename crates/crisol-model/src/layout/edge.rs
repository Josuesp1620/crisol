use serde::{Deserialize, Serialize};

/// One side of a container, used by anchors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Edge {
    Top,
    Left,
    Right,
    Bottom,
}

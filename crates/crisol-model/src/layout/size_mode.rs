use serde::{Deserialize, Serialize};

/// How a control takes its width or height inside a stack or a grid cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeMode {
    /// Exactly this many logical pixels.
    Fixed(i32),
    /// As small as its content allows.
    #[default]
    Hug,
    /// All the space the container leaves.
    Fill,
}

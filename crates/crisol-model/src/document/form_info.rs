use serde::{Deserialize, Serialize};

use crate::geometry::Size;
use crate::identity::TypeName;

/// What the form itself is: its Rust type, window title, design size and the user's state type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormInfo {
    pub name: TypeName,
    pub title: String,
    pub size: Size,
    /// Path of the user's state type stored in the generated struct (`crate::customers::CustomerState`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

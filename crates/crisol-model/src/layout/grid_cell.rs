use serde::{Deserialize, Serialize};

/// The cell a control occupies inside a grid, counted from zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GridCell {
    pub row: u32,
    pub column: u32,
    #[serde(
        default = "GridCell::single_span",
        skip_serializing_if = "GridCell::is_single_span"
    )]
    pub column_span: u32,
}

impl GridCell {
    pub fn new(row: u32, column: u32) -> Self {
        Self {
            row,
            column,
            column_span: 1,
        }
    }

    fn single_span() -> u32 {
        1
    }

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde passes fields by reference"
    )]
    fn is_single_span(span: &u32) -> bool {
        *span == 1
    }
}

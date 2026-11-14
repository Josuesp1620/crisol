use gpui::AnyElement;

use super::Length;

/// One child of a grid with its cell and width.
pub struct GridSlot {
    pub row: u32,
    pub column: u32,
    pub column_span: u32,
    pub width: Length,
    pub element: AnyElement,
}

use gpui::AnyElement;

use super::Length;

/// One child of a stack with the size it asked for.
pub struct StackSlot {
    pub width: Length,
    pub height: Length,
    pub element: AnyElement,
}

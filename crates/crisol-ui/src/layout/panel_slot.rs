use gpui::AnyElement;

use super::Anchors;

/// One child of an absolute panel: its designed rectangle and anchors.
pub struct PanelSlot {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub anchors: Anchors,
    pub element: AnyElement,
}

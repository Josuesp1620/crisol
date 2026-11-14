use gpui::{Div, Styled, px};

/// How a child takes its width or height in a container.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    /// Exactly this many logical pixels.
    Fixed(f32),
    /// As small as its content.
    Hug,
    /// All the space the container leaves.
    Fill,
}

impl Length {
    /// Applies the width to a slot. `along_flow` is true in containers that lay children out
    /// horizontally, where filling means growing; otherwise filling means stretching.
    pub fn apply_width(self, slot: Div, along_flow: bool) -> Div {
        match (self, along_flow) {
            (Length::Fixed(pixels), true) => slot.w(px(pixels)).flex_none(),
            (Length::Fixed(pixels), false) => slot.w(px(pixels)).self_start(),
            (Length::Fill, true) => slot.flex_1().min_w_0(),
            (Length::Fill, false) => slot.self_stretch(),
            (Length::Hug, true) => slot.flex_none(),
            (Length::Hug, false) => slot.self_start(),
        }
    }

    /// Applies the height to a slot. `along_flow` is true in containers that lay children
    /// out vertically, where filling means growing; otherwise filling means stretching.
    pub fn apply_height(self, slot: Div, along_flow: bool) -> Div {
        match (self, along_flow) {
            (Length::Fixed(pixels), _) => slot.h(px(pixels)),
            (Length::Fill, true) => slot.flex_1().min_h_0(),
            (Length::Fill, false) => slot.self_stretch(),
            (Length::Hug, _) => slot,
        }
    }
}

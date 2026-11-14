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
    /// Applies the width; `along_flow` is true when the container lays children out
    /// horizontally, where filling means growing instead of taking the full width.
    pub fn apply_width(self, slot: Div, along_flow: bool) -> Div {
        match (self, along_flow) {
            (Length::Fixed(pixels), _) => slot.w(px(pixels)).flex_none(),
            (Length::Fill, true) => slot.flex_1().min_w_0(),
            (Length::Fill, false) => slot.w_full(),
            (Length::Hug, _) => slot.flex_none(),
        }
    }

    /// Applies the height; `along_flow` is true when the container lays children out
    /// vertically, where filling means growing instead of taking the full height.
    pub fn apply_height(self, slot: Div, along_flow: bool) -> Div {
        match (self, along_flow) {
            (Length::Fixed(pixels), _) => slot.h(px(pixels)),
            (Length::Fill, true) => slot.flex_1().min_h_0(),
            (Length::Fill, false) => slot.h_full(),
            (Length::Hug, _) => slot,
        }
    }
}

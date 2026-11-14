use gpui::{
    App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::stack_slot::StackSlot;
use super::{Length, StackAlignment};
use crate::theme::Theme;

/// Places children one after another, vertically or horizontally.
#[derive(IntoElement)]
pub struct Stack {
    horizontal: bool,
    alignment: StackAlignment,
    gap: f32,
    padding: f32,
    fills_window: bool,
    slots: Vec<StackSlot>,
}

impl Stack {
    pub fn vertical() -> Self {
        Self::with_direction(false)
    }

    pub fn horizontal() -> Self {
        Self::with_direction(true)
    }

    fn with_direction(horizontal: bool) -> Self {
        Self {
            horizontal,
            alignment: StackAlignment::Start,
            gap: 0.,
            padding: 0.,
            fills_window: false,
            slots: Vec::new(),
        }
    }

    /// Takes the whole window, with the form's surface color: used by a form's root.
    pub fn fill(mut self) -> Self {
        self.fills_window = true;
        self
    }

    pub fn gap(mut self, pixels: f32) -> Self {
        self.gap = pixels;
        self
    }

    pub fn padding(mut self, pixels: f32) -> Self {
        self.padding = pixels;
        self
    }

    pub fn align(mut self, alignment: StackAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn child(self, element: impl IntoElement) -> Self {
        self.child_sized(Length::Hug, Length::Hug, element)
    }

    pub fn child_sized(mut self, width: Length, height: Length, element: impl IntoElement) -> Self {
        self.slots.push(StackSlot {
            width,
            height,
            element: element.into_any_element(),
        });
        self
    }
}

impl RenderOnce for Stack {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let horizontal = self.horizontal;
        let mut container = div()
            .flex()
            .when(horizontal, |container| container.flex_row().items_center())
            .when(!horizontal, |container| container.flex_col().items_start())
            .gap(px(self.gap))
            .p(px(self.padding))
            .when(self.fills_window, |container| {
                container
                    .size_full()
                    .bg(theme.colors.surface)
                    .text_color(theme.colors.ink)
                    .font_family(theme.fonts.body.clone())
                    .text_size(theme.metrics.text_size)
            });
        container = match self.alignment {
            StackAlignment::Start => container.justify_start(),
            StackAlignment::Center => container.justify_center(),
            StackAlignment::End => container.justify_end(),
            StackAlignment::SpaceBetween => container.justify_between(),
        };
        if !horizontal {
            container = container.w_full();
        }
        for slot in self.slots {
            let mut wrapper = div().flex();
            wrapper = slot.width.apply_width(wrapper, horizontal);
            wrapper = slot.height.apply_height(wrapper, !horizontal);
            container = container.child(wrapper.child(slot.element));
        }
        container
    }
}

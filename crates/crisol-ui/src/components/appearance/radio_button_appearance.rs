use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::theme::Theme;

/// A radio button: the ring, its dot and the text next to it.
#[derive(IntoElement)]
pub struct RadioButtonAppearance {
    text: SharedString,
    checked: bool,
    disabled: bool,
}

impl RadioButtonAppearance {
    pub fn new(text: impl Into<SharedString>, checked: bool, disabled: bool) -> Self {
        Self {
            text: text.into(),
            checked,
            disabled,
        }
    }
}

impl RenderOnce for RadioButtonAppearance {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let ring_color = if self.checked {
            theme.colors.accent
        } else {
            theme.colors.muted
        };
        let ring = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(theme.metrics.check_size)
            .rounded_full()
            .border(px(1.5))
            .border_color(ring_color)
            .bg(theme.colors.surface)
            .when(self.checked, |ring| {
                ring.child(div().size(px(8.)).rounded_full().bg(theme.colors.accent))
            });
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .text_color(theme.colors.ink)
            .whitespace_nowrap()
            .when(self.disabled, |row| row.opacity(0.5))
            .child(ring)
            .child(self.text)
    }
}

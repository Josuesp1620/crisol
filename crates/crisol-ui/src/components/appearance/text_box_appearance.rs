use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::theme::Theme;

/// A text box frame showing its text or placeholder, without editing.
#[derive(IntoElement)]
pub struct TextBoxAppearance {
    text: SharedString,
    placeholder: SharedString,
    disabled: bool,
}

impl TextBoxAppearance {
    pub fn new(
        text: impl Into<SharedString>,
        placeholder: impl Into<SharedString>,
        disabled: bool,
    ) -> Self {
        Self {
            text: text.into(),
            placeholder: placeholder.into(),
            disabled,
        }
    }
}

impl RenderOnce for TextBoxAppearance {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let (text, color) = if self.text.is_empty() {
            (self.placeholder, theme.colors.muted)
        } else {
            (self.text, theme.colors.ink)
        };
        div()
            .flex()
            .items_center()
            .min_w(px(60.))
            .h(theme.metrics.control_height)
            .px(theme.metrics.control_padding)
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.background)
            .text_color(color)
            .overflow_hidden()
            .when(self.disabled, |field| field.opacity(0.5))
            .child(div().truncate().child(text))
    }
}

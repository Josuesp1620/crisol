use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div};

use crate::theme::Theme;

/// Text that names or explains other controls.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_color(theme.colors.ink)
            .text_size(theme.metrics.text_size)
            .whitespace_nowrap()
            .child(self.text)
    }
}

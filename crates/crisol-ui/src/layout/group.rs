use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

use super::{Length, Stack};
use crate::theme::Theme;

/// A titled frame around related controls, stacked vertically.
#[derive(IntoElement)]
pub struct Group {
    title: SharedString,
    content: Stack,
}

impl Group {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            content: Stack::vertical(),
        }
    }

    pub fn gap(mut self, pixels: f32) -> Self {
        self.content = self.content.gap(pixels);
        self
    }

    pub fn padding(mut self, pixels: f32) -> Self {
        self.content = self.content.padding(pixels);
        self
    }

    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.content = self.content.child(element);
        self
    }

    pub fn child_sized(mut self, width: Length, height: Length, element: impl IntoElement) -> Self {
        self.content = self.content.child_sized(width, height, element);
        self
    }
}

impl RenderOnce for Group {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .w_full()
            .border_1()
            .border_color(theme.colors.line)
            .rounded(theme.metrics.panel_radius)
            .child(
                div()
                    .px_3()
                    .pt_2()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.muted)
                    .child(self.title),
            )
            .child(self.content)
    }
}

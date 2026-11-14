use crisol_ui::theme::Theme;
use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

/// An empty container on the canvas: a dashed area that shows where controls can go.
#[derive(IntoElement)]
pub struct EmptyContainer {
    kind: SharedString,
}

impl EmptyContainer {
    pub fn new(kind: impl Into<SharedString>) -> Self {
        Self { kind: kind.into() }
    }
}

impl RenderOnce for EmptyContainer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .min_w(px(160.))
            .h(px(56.))
            .border_1()
            .border_dashed()
            .border_color(theme.colors.muted)
            .rounded(theme.metrics.small_radius)
            .text_color(theme.colors.muted)
            .text_size(theme.metrics.small_text_size)
            .child(format!("{} · drop controls here", self.kind))
    }
}

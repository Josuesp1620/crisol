use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

/// A search box look-alike at the top of panes.
#[derive(IntoElement)]
pub struct SearchField {
    placeholder: SharedString,
}

impl SearchField {
    pub fn new(placeholder: impl Into<SharedString>) -> Self {
        Self {
            placeholder: placeholder.into(),
        }
    }
}

impl RenderOnce for SearchField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_center()
            .gap(px(6.))
            .m(px(8.))
            .px(px(8.))
            .py(px(4.))
            .rounded(theme.metrics.small_radius)
            .border_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.background)
            .text_color(theme.colors.muted)
            .child(Icon::new(IconName::Search).color(theme.colors.muted))
            .child(self.placeholder)
    }
}

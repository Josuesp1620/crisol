use crisol_ui::theme::Theme;
use gpui::{App, IntoElement, RenderOnce, Styled, Window, div, px};

/// The thin vertical line between groups of tool bar buttons.
#[derive(IntoElement)]
pub struct ToolBarSeparator;

impl RenderOnce for ToolBarSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_none()
            .w(px(1.))
            .h(px(20.))
            .mx(px(8.))
            .bg(Theme::of(cx).colors.line)
    }
}

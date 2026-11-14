use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, App, BoxShadow, FontWeight, IntoElement, ParentElement, Pixels, Styled, div, point,
    px,
};

/// The window around a form on the canvas: title bar, window buttons and the form body.
pub struct FormFrame {
    pub title: String,
    pub width: Pixels,
    pub height: Pixels,
    pub title_height: Pixels,
}

impl FormFrame {
    pub fn render(self, body: AnyElement, cx: &App) -> AnyElement {
        let theme = Theme::of(cx);
        let mut buttons = div()
            .ml_auto()
            .flex()
            .h_full()
            .text_color(theme.colors.muted);
        for icon in [IconName::Min, IconName::Max, IconName::X] {
            buttons = buttons.child(
                div()
                    .w(px(36.))
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(icon).color(theme.colors.muted)),
            );
        }
        div()
            .flex()
            .flex_col()
            .w(self.width)
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.line)
            .rounded(theme.metrics.panel_radius)
            .shadow(vec![BoxShadow {
                color: theme.colors.shadow,
                offset: point(px(0.), px(18.)),
                blur_radius: px(40.),
                spread_radius: px(-20.),
                inset: false,
            }])
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(8.))
                    .h(self.title_height)
                    .pl(px(12.))
                    .border_b_1()
                    .border_color(theme.colors.line)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(Icon::new(IconName::Form).color(theme.colors.ink))
                    .child(self.title)
                    .child(buttons),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .h(self.height)
                    .overflow_hidden()
                    .child(body),
            )
            .into_any_element()
    }
}

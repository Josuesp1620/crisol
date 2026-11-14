use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::designer_settings::DesignerSettings;

/// The title strip of a side pane, with its icon and pin.
#[derive(IntoElement)]
pub struct PaneHeader {
    icon: IconName,
    title: SharedString,
}

impl PaneHeader {
    pub fn new(icon: IconName, title: impl Into<SharedString>) -> Self {
        Self {
            icon,
            title: title.into(),
        }
    }
}

impl RenderOnce for PaneHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let settings = DesignerSettings::default();
        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .h(settings.pane_header_height)
            .pl(px(10.))
            .pr(px(8.))
            .bg(theme.colors.chrome)
            .border_b_1()
            .border_color(theme.colors.line)
            .text_size(theme.metrics.small_text_size)
            .font_weight(FontWeight::SEMIBOLD)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .child(Icon::new(self.icon))
                    .child(self.title),
            )
            .child(Icon::new(IconName::Pin).color(theme.colors.muted))
    }
}

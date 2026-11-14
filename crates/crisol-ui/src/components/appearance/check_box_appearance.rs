use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::icon::{Icon, IconName};
use crate::theme::Theme;

/// A check box: the box, its mark and the text next to it.
#[derive(IntoElement)]
pub struct CheckBoxAppearance {
    text: SharedString,
    checked: bool,
    disabled: bool,
}

impl CheckBoxAppearance {
    pub fn new(text: impl Into<SharedString>, checked: bool, disabled: bool) -> Self {
        Self {
            text: text.into(),
            checked,
            disabled,
        }
    }
}

impl RenderOnce for CheckBoxAppearance {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mark = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(theme.metrics.check_size)
            .rounded(px(4.))
            .when(self.checked, |mark| {
                mark.bg(theme.colors.accent).child(
                    Icon::new(IconName::Check)
                        .size(px(12.))
                        .color(theme.colors.on_accent),
                )
            })
            .when(!self.checked, |mark| {
                mark.bg(theme.colors.surface)
                    .border(px(1.5))
                    .border_color(theme.colors.muted)
            });
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .text_color(theme.colors.ink)
            .whitespace_nowrap()
            .when(self.disabled, |row| row.opacity(0.5))
            .child(mark)
            .child(self.text)
    }
}

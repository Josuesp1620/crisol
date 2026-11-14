use gpui::{
    App, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::icon::{Icon, IconName};
use crate::theme::Theme;

/// The closed combo box: the chosen option (or the placeholder) and the arrow.
#[derive(IntoElement)]
pub struct ComboBoxAppearance {
    text: Option<SharedString>,
    placeholder: SharedString,
    highlighted: bool,
    disabled: bool,
}

impl ComboBoxAppearance {
    pub fn new(text: Option<SharedString>, placeholder: impl Into<SharedString>) -> Self {
        Self {
            text,
            placeholder: placeholder.into(),
            highlighted: false,
            disabled: false,
        }
    }

    /// Draws the border in the selection color, as when focused or open.
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for ComboBoxAppearance {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let (text, color): (SharedString, Hsla) = match self.text {
            Some(text) => (text, theme.colors.ink),
            None => (self.placeholder, theme.colors.muted),
        };
        let border = if self.highlighted {
            theme.colors.selection
        } else {
            theme.colors.line
        };
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(6.))
            .min_w(px(80.))
            .h(theme.metrics.control_height)
            .px(theme.metrics.control_padding)
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(border)
            .bg(theme.colors.background)
            .text_color(color)
            .when(self.disabled, |trigger| trigger.opacity(0.5))
            .child(div().truncate().child(text))
            .child(Icon::new(IconName::Down).color(theme.colors.muted))
    }
}

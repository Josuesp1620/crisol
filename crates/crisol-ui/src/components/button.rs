use gpui::{
    App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use super::ButtonVariant;
use crate::theme::Theme;

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A button that runs an action when clicked.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    text: SharedString,
    variant: ButtonVariant,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            variant: ButtonVariant::Secondary,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let colors = &theme.colors;
        let (background, border, text) = match self.variant {
            ButtonVariant::Primary => (colors.accent, colors.accent, colors.on_accent),
            ButtonVariant::Secondary => (colors.surface, colors.line, colors.ink),
            ButtonVariant::Ghost => (
                gpui::transparent_black(),
                gpui::transparent_black(),
                colors.ink,
            ),
        };
        let hover_background = match self.variant {
            ButtonVariant::Primary => colors.accent_ink,
            _ => colors.sunken,
        };
        let disabled = self.disabled;
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .h(theme.metrics.button_height)
            .min_w(theme.metrics.button_min_width)
            .px(px(16.))
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(border)
            .bg(background)
            .text_color(text)
            .text_size(theme.metrics.text_size)
            .font_weight(FontWeight::SEMIBOLD)
            .whitespace_nowrap()
            .when(disabled, |button| button.opacity(0.5))
            .when(!disabled, |button| {
                button
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover_background))
            })
            .when_some(self.on_click.filter(|_| !disabled), |button, handler| {
                button.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(self.text)
    }
}

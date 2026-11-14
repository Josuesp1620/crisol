use gpui::{
    Context, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::RadioButtonEvent;
use crate::theme::Theme;

/// One option of a set where only one can be chosen. The form unchecks the others.
pub struct RadioButton {
    text: SharedString,
    checked: bool,
    disabled: bool,
}

impl EventEmitter<RadioButtonEvent> for RadioButton {}

impl RadioButton {
    pub fn new(_cx: &mut Context<Self>, text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            checked: false,
            disabled: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.checked != checked {
            self.checked = checked;
            cx.notify();
        }
    }

    fn choose(&mut self, cx: &mut Context<Self>) {
        if !self.disabled && !self.checked {
            self.checked = true;
            cx.emit(RadioButtonEvent::Checked);
            cx.notify();
        }
    }
}

impl Render for RadioButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        let size = theme.metrics.check_size;
        let ring_color = if self.checked {
            theme.colors.accent
        } else {
            theme.colors.muted
        };
        let ring = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(size)
            .rounded_full()
            .border(px(1.5))
            .border_color(ring_color)
            .bg(theme.colors.surface)
            .when(self.checked, |ring| {
                ring.child(div().size(px(8.)).rounded_full().bg(theme.colors.accent))
            });
        div()
            .id("radio-button")
            .flex()
            .items_center()
            .gap(px(8.))
            .text_color(theme.colors.ink)
            .when(self.disabled, |row| row.opacity(0.5))
            .when(!self.disabled, Styled::cursor_pointer)
            .on_click(cx.listener(|radio, _, _, cx| radio.choose(cx)))
            .child(ring)
            .child(self.text.clone())
    }
}

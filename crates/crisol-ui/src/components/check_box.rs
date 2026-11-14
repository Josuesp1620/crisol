use gpui::{
    Context, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::CheckBoxEvent;
use crate::icon::{Icon, IconName};
use crate::theme::Theme;

/// A yes or no choice with a text next to the box.
pub struct CheckBox {
    text: SharedString,
    checked: bool,
    disabled: bool,
}

impl EventEmitter<CheckBoxEvent> for CheckBox {}

impl CheckBox {
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
            cx.emit(CheckBoxEvent::Changed);
            cx.notify();
        }
    }

    fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_checked(!self.checked, cx);
        }
    }
}

impl Render for CheckBox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        let size = theme.metrics.check_size;
        let mark = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(size)
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
            .id("check-box")
            .flex()
            .items_center()
            .gap(px(8.))
            .text_color(theme.colors.ink)
            .when(self.disabled, |row| row.opacity(0.5))
            .when(!self.disabled, Styled::cursor_pointer)
            .on_click(cx.listener(|check_box, _, _, cx| check_box.toggle(cx)))
            .child(mark)
            .child(self.text.clone())
    }
}

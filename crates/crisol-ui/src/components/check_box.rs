use gpui::{
    Context, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};

use super::CheckBoxEvent;
use super::appearance::CheckBoxAppearance;

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
        div()
            .id("check-box")
            .when(!self.disabled, Styled::cursor_pointer)
            .on_click(cx.listener(|check_box, _, _, cx| check_box.toggle(cx)))
            .child(CheckBoxAppearance::new(
                self.text.clone(),
                self.checked,
                self.disabled,
            ))
    }
}

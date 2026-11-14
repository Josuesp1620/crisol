use gpui::{
    Context, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};

use super::RadioButtonEvent;
use super::appearance::RadioButtonAppearance;

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
        div()
            .id("radio-button")
            .when(!self.disabled, Styled::cursor_pointer)
            .on_click(cx.listener(|radio, _, _, cx| radio.choose(cx)))
            .child(RadioButtonAppearance::new(
                self.text.clone(),
                self.checked,
                self.disabled,
            ))
    }
}

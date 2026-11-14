use gpui::{
    Context, EventEmitter, FocusHandle, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, StatefulInteractiveElement, Styled, Window, deferred, div,
    prelude::FluentBuilder, px,
};

use super::{ComboBoxEvent, FocusTracker};
use crate::icon::{Icon, IconName};
use crate::theme::Theme;

/// A drop-down list to pick one option.
pub struct ComboBox {
    items: Vec<SharedString>,
    selected: Option<usize>,
    placeholder: SharedString,
    disabled: bool,
    open: bool,
    focus_handle: FocusHandle,
    focus_tracker: FocusTracker,
}

impl EventEmitter<ComboBoxEvent> for ComboBox {}

impl ComboBox {
    pub fn new(cx: &mut Context<Self>, items: Vec<&str>) -> Self {
        let mut owned = Vec::with_capacity(items.len());
        for item in items {
            owned.push(SharedString::from(item.to_owned()));
        }
        Self {
            items: owned,
            selected: None,
            placeholder: SharedString::default(),
            disabled: false,
            open: false,
            focus_handle: cx.focus_handle(),
            focus_tracker: FocusTracker::default(),
        }
    }

    /// Selects the option with this text when the form opens.
    pub fn selected(mut self, text: &str) -> Self {
        self.selected = self.items.iter().position(|item| item.as_ref() == text);
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_text(&self) -> Option<&str> {
        self.selected
            .and_then(|index| self.items.get(index))
            .map(SharedString::as_ref)
    }

    pub fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        self.open = false;
        if self.selected != Some(index) && index < self.items.len() {
            self.selected = Some(index);
            cx.emit(ComboBoxEvent::SelectionChanged);
        }
        cx.notify();
    }

    fn toggle(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.open = !self.open;
        window.focus(&self.focus_handle, cx);
        if self.open {
            cx.emit(ComboBoxEvent::Opened);
        }
        cx.notify();
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        if self.open {
            self.open = false;
            cx.notify();
        }
    }

    fn render_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut list = div()
            .absolute()
            .top(px(34.))
            .left_0()
            .w_full()
            .py(px(4.))
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.line)
            .rounded(theme.metrics.control_radius)
            .shadow_lg()
            .occlude();
        for (index, item) in self.items.iter().enumerate() {
            let chosen = self.selected == Some(index);
            list = list.child(
                div()
                    .id(index)
                    .px(theme.metrics.control_padding)
                    .py(px(5.))
                    .cursor_pointer()
                    .when(chosen, |row| {
                        row.bg(theme.colors.selection_soft)
                            .text_color(theme.colors.selection)
                    })
                    .hover(|style| style.bg(theme.colors.sunken))
                    .on_click(cx.listener(move |combo, _, _, cx| combo.select(index, cx)))
                    .child(item.clone()),
            );
        }
        deferred(list)
    }
}

impl Render for ComboBox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = self.focus_handle.clone();
        self.focus_tracker.watch(
            &handle,
            window,
            cx,
            |_, cx| cx.emit(ComboBoxEvent::FocusGained),
            |combo, cx| {
                combo.close(cx);
                cx.emit(ComboBoxEvent::FocusLost);
            },
        );
        let theme = Theme::of(cx);
        let (text, text_color) = match self.selected_text() {
            Some(selected) => (SharedString::from(selected.to_owned()), theme.colors.ink),
            None => (self.placeholder.clone(), theme.colors.muted),
        };
        let focused = self.focus_handle.is_focused(window);
        let border = if focused || self.open {
            theme.colors.selection
        } else {
            theme.colors.line
        };
        let muted = theme.colors.muted;
        div()
            .relative()
            .w_full()
            .min_w(px(80.))
            .child(
                div()
                    .id("combo-box")
                    .track_focus(&self.focus_handle)
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(6.))
                    .h(theme.metrics.control_height)
                    .px(theme.metrics.control_padding)
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(border)
                    .bg(theme.colors.background)
                    .text_color(text_color)
                    .when(self.disabled, |trigger| trigger.opacity(0.5))
                    .when(!self.disabled, Styled::cursor_pointer)
                    .on_click(cx.listener(|combo, _, window, cx| combo.toggle(window, cx)))
                    .child(div().truncate().child(text))
                    .child(Icon::new(IconName::Down).color(muted)),
            )
            .when(self.open, |container| container.child(self.render_list(cx)))
    }
}

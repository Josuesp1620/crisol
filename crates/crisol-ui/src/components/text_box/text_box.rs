use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, EntityInputHandler, EventEmitter,
    FocusHandle, Focusable, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Point, Render, ShapedLine, SharedString,
    Styled, UTF16Selection, Window, div, prelude::FluentBuilder,
};

use super::text_box_actions::{
    Backspace, Copy, Cut, Delete, MoveEnd, MoveHome, MoveLeft, MoveRight, Paste, SelectAll,
    SelectEnd, SelectHome, SelectLeft, SelectRight, Submit, TextBoxKeyBindings,
};
use super::text_box_element::TextBoxElement;
use super::{InputKind, TextBoxEvent, TextEditState};
use crate::components::FocusTracker;
use crate::theme::Theme;

/// A field where the user types one line of text.
pub struct TextBox {
    state: TextEditState,
    placeholder: SharedString,
    read_only: bool,
    disabled: bool,
    focus_handle: FocusHandle,
    focus_tracker: FocusTracker,
    selecting: bool,
    pub(super) last_line: Option<ShapedLine>,
    pub(super) last_bounds: Option<Bounds<Pixels>>,
}

impl EventEmitter<TextBoxEvent> for TextBox {}

impl Focusable for TextBox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl TextBox {
    const MASK: char = '•';

    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            state: TextEditState::new(InputKind::Text, 0),
            placeholder: SharedString::default(),
            read_only: false,
            disabled: false,
            focus_handle: cx.focus_handle(),
            focus_tracker: FocusTracker::default(),
            selecting: false,
            last_line: None,
            last_bounds: None,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.state.set_content(text);
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    pub fn input_kind(mut self, input_kind: InputKind) -> Self {
        self.state.set_input_kind(input_kind);
        self
    }

    pub fn max_length(mut self, characters: usize) -> Self {
        self.state.set_max_length(characters);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The current text.
    pub fn value(&self) -> &str {
        self.state.content()
    }

    pub fn set_value(&mut self, text: &str, cx: &mut Context<Self>) {
        self.state.set_content(text);
        cx.emit(TextBoxEvent::Changed);
        cx.notify();
    }

    pub(super) fn edit_state(&self) -> &TextEditState {
        &self.state
    }

    pub(super) fn placeholder_text(&self) -> SharedString {
        self.placeholder.clone()
    }

    pub(super) fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    pub(super) fn focus(&self) -> &FocusHandle {
        &self.focus_handle
    }

    /// The text as drawn: dots for passwords.
    pub(super) fn display_text(&self) -> SharedString {
        if self.state.input_kind().masks_text() {
            let count = self.state.content().chars().count();
            SharedString::from(Self::MASK.to_string().repeat(count))
        } else {
            SharedString::from(self.state.content().to_owned())
        }
    }

    pub(super) fn display_offset(&self, offset: usize) -> usize {
        if self.state.input_kind().masks_text() {
            self.state.content()[..offset].chars().count() * Self::MASK.len_utf8()
        } else {
            offset
        }
    }

    fn content_offset(&self, display_offset: usize) -> usize {
        if !self.state.input_kind().masks_text() {
            return display_offset;
        }
        let characters = display_offset / Self::MASK.len_utf8();
        match self.state.content().char_indices().nth(characters) {
            Some((index, _)) => index,
            None => self.state.content().len(),
        }
    }

    fn editable(&self) -> bool {
        !self.read_only && !self.disabled
    }

    fn after_edit(&mut self, changed: bool, window: &mut Window, cx: &mut Context<Self>) {
        window.notify_text_input_changed(&self.focus_handle);
        if changed {
            cx.emit(TextBoxEvent::Changed);
        }
        cx.notify();
    }

    fn after_move(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        window.notify_text_selection_changed(&self.focus_handle);
        cx.notify();
    }

    fn offset_at(&self, position: Point<Pixels>) -> usize {
        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_line.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.state.content().len();
        }
        self.content_offset(line.closest_index_for_x(position.x - bounds.left()))
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if self.state.selection().is_empty() {
            let previous = self.state.previous_boundary(self.state.cursor());
            self.state.select_to(previous);
        }
        let changed = self.state.replace(None, "");
        self.after_edit(changed, window, cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if self.state.selection().is_empty() {
            let next = self.state.next_boundary(self.state.cursor());
            self.state.select_to(next);
        }
        let changed = self.state.replace(None, "");
        self.after_edit(changed, window, cx);
    }

    fn move_left(&mut self, _: &MoveLeft, window: &mut Window, cx: &mut Context<Self>) {
        let selection = self.state.selection();
        let target = if selection.is_empty() {
            self.state.previous_boundary(self.state.cursor())
        } else {
            selection.start
        };
        self.state.move_to(target);
        self.after_move(window, cx);
    }

    fn move_right(&mut self, _: &MoveRight, window: &mut Window, cx: &mut Context<Self>) {
        let selection = self.state.selection();
        let target = if selection.is_empty() {
            self.state.next_boundary(selection.end)
        } else {
            selection.end
        };
        self.state.move_to(target);
        self.after_move(window, cx);
    }

    fn select_left(&mut self, _: &SelectLeft, window: &mut Window, cx: &mut Context<Self>) {
        self.state
            .select_to(self.state.previous_boundary(self.state.cursor()));
        self.after_move(window, cx);
    }

    fn select_right(&mut self, _: &SelectRight, window: &mut Window, cx: &mut Context<Self>) {
        self.state
            .select_to(self.state.next_boundary(self.state.cursor()));
        self.after_move(window, cx);
    }

    fn select_all(&mut self, _: &SelectAll, window: &mut Window, cx: &mut Context<Self>) {
        self.state.select_all();
        self.after_move(window, cx);
    }

    fn move_home(&mut self, _: &MoveHome, window: &mut Window, cx: &mut Context<Self>) {
        self.state.move_to(0);
        self.after_move(window, cx);
    }

    fn move_end(&mut self, _: &MoveEnd, window: &mut Window, cx: &mut Context<Self>) {
        self.state.move_to(self.state.content().len());
        self.after_move(window, cx);
    }

    fn select_home(&mut self, _: &SelectHome, window: &mut Window, cx: &mut Context<Self>) {
        self.state.select_to(0);
        self.after_move(window, cx);
    }

    fn select_end(&mut self, _: &SelectEnd, window: &mut Window, cx: &mut Context<Self>) {
        self.state.select_to(self.state.content().len());
        self.after_move(window, cx);
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let changed = self.state.replace(None, &text);
            self.after_edit(changed, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.state.selection().is_empty() && !self.state.input_kind().masks_text() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.state.selected_text().to_owned(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.editable() || self.state.selection().is_empty() {
            return;
        }
        self.copy(&Copy, window, cx);
        let changed = self.state.replace(None, "");
        self.after_edit(changed, window, cx);
    }

    #[allow(
        clippy::unused_self,
        reason = "action handlers receive the control like the others"
    )]
    fn submit(&mut self, _: &Submit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(TextBoxEvent::Submitted);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        self.selecting = true;
        window.focus(&self.focus_handle, cx);
        let offset = self.offset_at(event.position);
        if event.modifiers.shift {
            self.state.select_to(offset);
        } else {
            self.state.move_to(offset);
        }
        self.after_move(window, cx);
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.selecting = false;
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selecting {
            self.state.select_to(self.offset_at(event.position));
            self.after_move(window, cx);
        }
    }
}

impl EntityInputHandler for TextBox {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.state.range_from_utf16(&range_utf16);
        actual_range.replace(self.state.range_to_utf16(&range));
        Some(self.state.content()[range].to_owned())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.state.range_to_utf16(&self.state.selection()),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.state
            .marked()
            .map(|range| self.state.range_to_utf16(&range))
    }

    fn unmark_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state.unmark();
        self.after_edit(false, window, cx);
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16.map(|range| self.state.range_from_utf16(&range));
        let changed = self.state.replace(range, text);
        self.after_edit(changed, window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editable() {
            return;
        }
        let range = range_utf16.map(|range| self.state.range_from_utf16(&range));
        let changed = self
            .state
            .replace_and_mark(range, text, new_selected_range_utf16);
        self.after_edit(changed, window, cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let line = self.last_line.as_ref()?;
        let range = self.state.range_from_utf16(&range_utf16);
        let start = line.x_for_index(self.display_offset(range.start));
        let end = line.x_for_index(self.display_offset(range.end));
        Some(Bounds::from_corners(
            gpui::point(bounds.left() + start, bounds.top()),
            gpui::point(bounds.left() + end, bounds.bottom()),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let local = self.last_bounds?.localize(&point)?;
        let line = self.last_line.as_ref()?;
        let offset = self.content_offset(line.closest_index_for_x(local.x));
        Some(self.state.offset_to_utf16(offset))
    }
}

impl Render for TextBox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = self.focus_handle.clone();
        self.focus_tracker.watch(
            &handle,
            window,
            cx,
            |_, cx| cx.emit(TextBoxEvent::FocusGained),
            |_, cx| cx.emit(TextBoxEvent::FocusLost),
        );
        let theme = Theme::of(cx);
        let focused = self.focus_handle.is_focused(window);
        let border = if focused {
            theme.colors.selection
        } else {
            theme.colors.line
        };
        div()
            .key_context(TextBoxKeyBindings::CONTEXT)
            .track_focus(&self.focus_handle)
            .flex()
            .items_center()
            .w_full()
            .min_w(gpui::px(60.))
            .h(theme.metrics.control_height)
            .px(theme.metrics.control_padding)
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(border)
            .bg(theme.colors.background)
            .text_color(theme.colors.ink)
            .text_size(theme.metrics.text_size)
            .overflow_hidden()
            .when(self.disabled, |field| field.opacity(0.5))
            .when(!self.disabled, |field| field.cursor(CursorStyle::IBeam))
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::move_left))
            .on_action(cx.listener(Self::move_right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::move_home))
            .on_action(cx.listener(Self::move_end))
            .on_action(cx.listener(Self::select_home))
            .on_action(cx.listener(Self::select_end))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::submit))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .child(TextBoxElement::new(cx.entity()))
    }
}

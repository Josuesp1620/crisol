use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::ClickHandler;

/// A tab of an open document, with a dot when it has unsaved changes.
#[derive(IntoElement)]
pub struct DocumentTab {
    id: ElementId,
    icon: IconName,
    label: SharedString,
    active: bool,
    dirty: bool,
    on_click: Option<ClickHandler>,
    on_close: Option<ClickHandler>,
}

impl DocumentTab {
    pub fn new(id: impl Into<ElementId>, icon: IconName, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon,
            label: label.into(),
            active: false,
            dirty: false,
            on_click: None,
            on_close: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn on_click(mut self, handler: ClickHandler) -> Self {
        self.on_click = Some(handler);
        self
    }

    /// Shows a close button that runs `handler`.
    pub fn on_close(mut self, handler: ClickHandler) -> Self {
        self.on_close = Some(handler);
        self
    }
}

impl RenderOnce for DocumentTab {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let color = if self.active {
            theme.colors.ink
        } else {
            theme.colors.muted
        };
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(6.))
            .px(px(12.))
            .h_full()
            .whitespace_nowrap()
            .border_r_1()
            .border_color(theme.colors.line)
            .text_color(color)
            .relative()
            .when(self.active, |tab| {
                tab.bg(theme.colors.surface).child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .right_0()
                        .h(px(2.))
                        .bg(theme.colors.accent),
                )
            })
            .when_some(self.on_click, |tab, handler| {
                tab.cursor_pointer()
                    .on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(Icon::new(self.icon).color(color))
            .child(self.label)
            .when(self.dirty, |tab| {
                tab.child(div().size(px(7.)).rounded_full().bg(theme.colors.accent))
            })
            .when_some(self.on_close, |tab, handler| {
                let hover = theme.colors.sunken;
                tab.child(
                    div()
                        .id("close")
                        .rounded(px(3.))
                        .cursor_pointer()
                        .hover(move |style| style.bg(hover))
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx);
                        })
                        .child(
                            Icon::new(IconName::X)
                                .size(px(12.))
                                .color(theme.colors.muted),
                        ),
                )
            })
    }
}

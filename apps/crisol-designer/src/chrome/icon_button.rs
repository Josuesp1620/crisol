use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::{ClickHandler, TooltipView};

/// A square button showing one icon, used in tool bars and pane headers.
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    active: bool,
    dimmed: bool,
    tooltip: Option<SharedString>,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            active: false,
            dimmed: false,
            tooltip: None,
            on_click: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Shown but not available, like alignment with a single control selected.
    pub fn dimmed(mut self, dimmed: bool) -> Self {
        self.dimmed = dimmed;
        self
    }

    /// What the button does, shown when the pointer rests on it.
    pub fn tooltip(mut self, text: impl Into<SharedString>) -> Self {
        self.tooltip = Some(text.into());
        self
    }

    pub fn on_click(mut self, handler: ClickHandler) -> Self {
        self.on_click = Some(handler);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let color = if self.active {
            theme.colors.selection
        } else {
            theme.colors.ink
        };
        let hover = theme.colors.sunken;
        let dimmed = self.dimmed;
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(28.))
            .rounded(theme.metrics.small_radius)
            .when(self.active, |button| button.bg(theme.colors.selection_soft))
            .when(dimmed, |button| button.opacity(0.45))
            .when(!dimmed && !self.active, |button| {
                button.hover(move |style| style.bg(hover))
            })
            .when(!dimmed, Styled::cursor_pointer)
            .when_some(self.tooltip, |button, text| {
                button.tooltip(TooltipView::builder(text))
            })
            .when_some(self.on_click.filter(|_| !dimmed), |button, handler| {
                button.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(Icon::new(self.icon).color(color))
    }
}

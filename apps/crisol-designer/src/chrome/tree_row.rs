use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::ClickHandler;
use crate::designer_settings::DesignerSettings;

/// One line of a tree: indentation, an optional chevron, an icon, a label and a trailing mark.
#[derive(IntoElement)]
pub struct TreeRow {
    id: ElementId,
    depth: usize,
    expanded: Option<bool>,
    icon: IconName,
    label: SharedString,
    selected: bool,
    dimmed: bool,
    trailing: Option<AnyElement>,
    on_click: Option<ClickHandler>,
}

impl TreeRow {
    const INDENT: f32 = 14.;
    const BASE_PADDING: f32 = 10.;

    pub fn new(
        id: impl Into<ElementId>,
        depth: usize,
        icon: IconName,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            depth,
            expanded: None,
            icon,
            label: label.into(),
            selected: false,
            dimmed: false,
            trailing: None,
            on_click: None,
        }
    }

    /// Shows a chevron: down when expanded, right when collapsed.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn dimmed(mut self, dimmed: bool) -> Self {
        self.dimmed = dimmed;
        self
    }

    pub fn trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }

    pub fn on_click(mut self, handler: ClickHandler) -> Self {
        self.on_click = Some(handler);
        self
    }
}

impl RenderOnce for TreeRow {
    #[allow(clippy::cast_precision_loss, reason = "tree depth is a small number")]
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let settings = DesignerSettings::default();
        let color = if self.selected {
            theme.colors.selection
        } else if self.dimmed {
            theme.colors.muted
        } else {
            theme.colors.ink
        };
        let chevron = match self.expanded {
            Some(true) => Icon::new(IconName::Down)
                .color(theme.colors.muted)
                .into_any_element(),
            Some(false) => Icon::new(IconName::Right)
                .color(theme.colors.muted)
                .into_any_element(),
            None => div().w(px(15.)).flex_none().into_any_element(),
        };
        let hover = theme.colors.sunken;
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(6.))
            .h(settings.row_height - px(2.))
            .pl(px(Self::BASE_PADDING + self.depth as f32 * Self::INDENT))
            .pr(px(8.))
            .whitespace_nowrap()
            .text_color(color)
            .when(self.selected, |row| row.bg(theme.colors.selection_soft))
            .when(!self.selected, |row| {
                row.hover(move |style| style.bg(hover))
            })
            .when_some(self.on_click, |row, handler| {
                row.cursor_pointer()
                    .on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(chevron)
            .child(Icon::new(self.icon).color(color))
            .child(div().flex_1().truncate().child(self.label))
            .when_some(self.trailing, gpui::ParentElement::child)
    }
}

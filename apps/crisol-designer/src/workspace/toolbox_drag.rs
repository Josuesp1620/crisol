use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, div, px};

/// A control being dragged from the toolbox.
#[derive(Debug, Clone)]
pub struct ToolboxDrag {
    pub kind: &'static str,
    pub label: SharedString,
    pub icon: IconName,
}

/// What follows the pointer while dragging from the toolbox.
pub struct ToolboxDragPreview {
    pub drag: ToolboxDrag,
}

impl Render for ToolboxDragPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(10.))
            .py(px(5.))
            .rounded(theme.metrics.small_radius)
            .border_1()
            .border_color(theme.colors.selection)
            .bg(theme.colors.selection_soft)
            .text_color(theme.colors.selection)
            .text_size(theme.metrics.text_size)
            .child(Icon::new(self.drag.icon).color(theme.colors.selection))
            .child(self.drag.label.clone())
    }
}

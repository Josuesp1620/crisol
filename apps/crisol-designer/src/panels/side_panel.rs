use crisol_ui::theme::Theme;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div};

use super::{ExplorerPanel, PropertyGridPanel};
use crate::workspace::DesignerWorkspace;

/// The right column: project explorer above the property grid.
pub struct SidePanel<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> SidePanel<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .flex_none()
            .w(self.workspace.settings().side_panel_width)
            .h_full()
            .bg(theme.colors.surface)
            .border_l_1()
            .border_color(theme.colors.line)
            .child(ExplorerPanel::new(self.workspace).render(window, cx))
            .child(PropertyGridPanel::new(self.workspace).render(window, cx))
            .into_any_element()
    }
}

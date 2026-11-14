use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, MouseDownEvent, ParentElement,
    StatefulInteractiveElement, Styled, Window, deferred, div, prelude::FluentBuilder, px,
};

use crate::chrome::KeyboardHint;
use crate::workspace::{DesignerWorkspace, MenuCommand};

/// The list of commands under an open menu title.
pub struct MenuDropdown<'workspace> {
    workspace: &'workspace DesignerWorkspace,
    menu: usize,
}

impl<'workspace> MenuDropdown<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace, menu: usize) -> Self {
        Self { workspace, menu }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let title_height = self.workspace.settings().title_bar_height;
        let mut list = div()
            .absolute()
            .top(px(28.))
            .left_0()
            .w(self.workspace.settings().menu_width)
            .py(px(4.))
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.line)
            .rounded(theme.metrics.panel_radius)
            .shadow_lg()
            .occlude()
            .on_mouse_down_out(
                cx.listener(move |workspace, event: &MouseDownEvent, _, cx| {
                    if event.position.y > title_height {
                        workspace.close_menu(cx);
                    }
                }),
            );
        let items = MenuCommand::items_for(self.menu);
        if items.is_empty() {
            list = list.child(
                div()
                    .px(px(12.))
                    .py(px(6.))
                    .text_color(theme.colors.muted)
                    .child("Coming in a later phase"),
            );
        }
        for command in items {
            let command = *command;
            let shortcut = command.shortcut();
            list = list.child(
                div()
                    .id(command.label())
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(12.))
                    .py(px(6.))
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.colors.selection_soft))
                    .on_click(cx.listener(move |workspace, _, window, cx| {
                        workspace.run_menu_command(command, window, cx);
                    }))
                    .child(command.label())
                    .when(!shortcut.is_empty(), |item| {
                        item.child(KeyboardHint::new(shortcut))
                    }),
            );
        }
        deferred(list).with_priority(1).into_any_element()
    }
}

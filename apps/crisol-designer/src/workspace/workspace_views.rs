use crisol_model::identity::NodeId;
use gpui::{Context, Focusable, PromptLevel, Window};

use super::{DesignerWorkspace, MenuCommand};
use crate::designer_texts::DesignerTexts;

/// Panels that fold, filter and switch: toolbox groups, property categories, the bottom
/// panel, the command palette, the object list, tabs and the start page.
impl DesignerWorkspace {
    pub fn toggle_group(&mut self, group: &str, cx: &mut Context<Self>) {
        if !self.collapsed_groups.remove(group) {
            self.collapsed_groups.insert(group.to_owned());
        }
        cx.notify();
    }

    pub fn toggle_category(&mut self, category: &str, cx: &mut Context<Self>) {
        if !self.collapsed_categories.remove(category) {
            self.collapsed_categories.insert(category.to_owned());
        }
        cx.notify();
    }

    pub fn set_alphabetical(&mut self, alphabetical: bool, cx: &mut Context<Self>) {
        self.alphabetical = alphabetical;
        cx.notify();
    }

    pub fn toggle_object_list(&mut self, cx: &mut Context<Self>) {
        self.object_list_open = !self.object_list_open;
        cx.notify();
    }

    pub fn choose_object(&mut self, node: NodeId, cx: &mut Context<Self>) {
        self.object_list_open = false;
        self.select(node, false, cx);
    }

    pub fn toggle_bottom_panel(&mut self, cx: &mut Context<Self>) {
        self.bottom_collapsed = !self.bottom_collapsed;
        cx.notify();
    }

    /// Selects the control a diagnostic is about.
    pub fn reveal(&mut self, node: Option<NodeId>, cx: &mut Context<Self>) {
        if let Some(node) = node
            && self.session().document().contains(node)
        {
            self.select(node, false, cx);
        }
    }

    pub fn toggle_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.palette_open = !self.palette_open;
        self.open_menu = None;
        if self.palette_open {
            self.palette_search
                .update(cx, |field, cx| field.set_value("", cx));
            window.focus(&self.palette_search.focus_handle(cx), cx);
        }
        cx.notify();
    }

    /// Commands whose name contains what was typed in the palette.
    pub fn palette_commands(&self, cx: &gpui::App) -> Vec<MenuCommand> {
        let query = self.palette_search.read(cx).value().to_lowercase();
        let mut found = Vec::new();
        for command in MenuCommand::ALL {
            if command.label().to_lowercase().contains(&query) {
                found.push(command);
            }
        }
        found
    }

    pub fn run_palette_command(
        &mut self,
        command: MenuCommand,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.palette_open = false;
        self.run_menu_command(command, window, cx);
    }

    pub fn show_start_page(&mut self, cx: &mut Context<Self>) {
        self.show_start = true;
        self.property_editor = None;
        self.explorer.clear();
        cx.notify();
    }

    /// Closes a tab, asking first when it has unsaved changes.
    pub fn close_tab(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.sessions.len() {
            return;
        }
        if !self.sessions[index].has_unsaved_changes() || self.show_start {
            self.remove_tab(index, cx);
            return;
        }
        self.activate(index, cx);
        let answer = window.prompt(
            PromptLevel::Warning,
            DesignerTexts::SAVE_CHANGES,
            Some(&self.sessions[index].file_name()),
            &[
                DesignerTexts::SAVE,
                DesignerTexts::DISCARD,
                DesignerTexts::CANCEL,
            ],
            cx,
        );
        cx.spawn_in(window, async move |workspace, cx| {
            let Ok(choice) = answer.await else {
                return;
            };
            let _ = workspace.update_in(cx, |workspace, window, cx| match choice {
                0 => {
                    workspace.save_form(window, cx);
                    if !workspace.sessions[workspace.active].has_unsaved_changes() {
                        workspace.remove_tab(index, cx);
                    }
                }
                1 => workspace.remove_tab(index, cx),
                _ => {}
            });
        })
        .detach();
    }

    fn remove_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.sessions.len() == 1 {
            self.placeholder = true;
            self.show_start_page(cx);
            return;
        }
        self.sessions.remove(index);
        let next = if self.active >= self.sessions.len() {
            self.sessions.len() - 1
        } else {
            self.active
        };
        self.activate(next, cx);
    }
}

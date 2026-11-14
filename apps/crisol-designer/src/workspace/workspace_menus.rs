use gpui::{Context, Window};

use super::{DesignerWorkspace, MenuCommand};
use crate::designer_texts::DesignerTexts;

/// The menu bar: opening menus and running their commands.
impl DesignerWorkspace {
    pub fn toggle_menu(&mut self, index: usize, cx: &mut Context<Self>) {
        self.open_menu = if self.open_menu == Some(index) {
            None
        } else {
            Some(index)
        };
        cx.notify();
    }

    /// Moves an open menu to `index` when the pointer passes over another title.
    pub fn hover_menu(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.open_menu.is_some() && self.open_menu != Some(index) {
            self.open_menu = Some(index);
            cx.notify();
        }
    }

    pub fn close_menu(&mut self, cx: &mut Context<Self>) {
        if self.open_menu.take().is_some() {
            cx.notify();
        }
    }

    pub fn run_menu_command(
        &mut self,
        command: MenuCommand,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_menu = None;
        match command {
            MenuCommand::NewProject => self.open_new_project(window, cx),
            MenuCommand::AddForm => self.add_form(cx),
            MenuCommand::SaveAll => self.save_all(window, cx),
            MenuCommand::ViewCode => self.view_code(None, cx),
            MenuCommand::ViewDesigner => self.view_designer(cx),
            MenuCommand::BuildProject => self.build_project(cx),
            MenuCommand::RunProject => self.run_project(cx),
            MenuCommand::NewForm => self.new_form(window, cx),
            MenuCommand::OpenForm => self.open_form(window, cx),
            MenuCommand::SaveForm => self.save_form(window, cx),
            MenuCommand::SaveFormAs => self.save_form_as(window, cx),
            MenuCommand::CloseTab => self.close_tab(self.active, window, cx),
            MenuCommand::GenerateCode => self.generate_code(cx),
            MenuCommand::Quit => cx.quit(),
            MenuCommand::Undo => self.undo(cx),
            MenuCommand::Redo => self.redo(cx),
            MenuCommand::Copy => self.copy_selection(cx),
            MenuCommand::Paste => self.paste_clipboard(cx),
            MenuCommand::Duplicate => self.duplicate_selection(cx),
            MenuCommand::Delete => self.delete_selection(cx),
            MenuCommand::SelectParent => self.select_parent(cx),
            MenuCommand::ToggleTheme => self.toggle_theme(cx),
            MenuCommand::ShowStart => self.show_start_page(cx),
            MenuCommand::Arrange(alignment) => self.arrange_selection(alignment, cx),
            MenuCommand::RunPreview => self.run_preview(cx),
            MenuCommand::About => {
                self.output.info(format!(
                    "{} {} · {} · {}",
                    DesignerTexts::APP_NAME,
                    env!("CARGO_PKG_VERSION"),
                    DesignerTexts::GPUI_REVISION,
                    DesignerTexts::RENDERER
                ));
                self.show_bottom_tab(super::BottomTab::Output, cx);
            }
        }
        cx.notify();
    }
}

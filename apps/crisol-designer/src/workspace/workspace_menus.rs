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
            MenuCommand::NewForm => self.new_form(window, cx),
            MenuCommand::OpenForm => self.open_form(window, cx),
            MenuCommand::SaveForm => self.save_form(window, cx),
            MenuCommand::SaveFormAs => self.save_form_as(window, cx),
            MenuCommand::GenerateCode => self.generate_code(cx),
            MenuCommand::Quit => cx.quit(),
            MenuCommand::Undo => self.undo(cx),
            MenuCommand::Redo => self.redo(cx),
            MenuCommand::Delete => self.delete_selection(cx),
            MenuCommand::ToggleTheme => self.toggle_theme(cx),
            MenuCommand::RunPreview => self.run_preview(cx),
            MenuCommand::About => {
                self.output.info(format!(
                    "{} {} · {} · {}",
                    DesignerTexts::APP_NAME,
                    env!("CARGO_PKG_VERSION"),
                    DesignerTexts::GPUI_REVISION,
                    DesignerTexts::RENDERER
                ));
                self.bottom_tab = super::BottomTab::Output;
            }
        }
        cx.notify();
    }
}

use crate::designer_texts::DesignerTexts;

/// Everything the menu bar can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommand {
    NewForm,
    OpenForm,
    SaveForm,
    SaveFormAs,
    GenerateCode,
    Quit,
    Undo,
    Redo,
    Delete,
    ToggleTheme,
    RunPreview,
    About,
}

impl MenuCommand {
    /// The commands of each menu, in the order of `DesignerTexts::MENUS`.
    pub fn items_for(menu: usize) -> &'static [MenuCommand] {
        match menu {
            0 => &[
                MenuCommand::NewForm,
                MenuCommand::OpenForm,
                MenuCommand::SaveForm,
                MenuCommand::SaveFormAs,
                MenuCommand::GenerateCode,
                MenuCommand::Quit,
            ],
            1 => &[MenuCommand::Undo, MenuCommand::Redo, MenuCommand::Delete],
            2 => &[MenuCommand::ToggleTheme],
            4 => &[MenuCommand::GenerateCode],
            5 => &[MenuCommand::RunPreview],
            6 => &[MenuCommand::About],
            _ => &[],
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MenuCommand::NewForm => "New form",
            MenuCommand::OpenForm => "Open…",
            MenuCommand::SaveForm => "Save",
            MenuCommand::SaveFormAs => "Save as…",
            MenuCommand::GenerateCode => "Generate code",
            MenuCommand::Quit => "Exit",
            MenuCommand::Undo => "Undo",
            MenuCommand::Redo => "Redo",
            MenuCommand::Delete => "Delete",
            MenuCommand::ToggleTheme => "Light / dark theme",
            MenuCommand::RunPreview => "Run preview",
            MenuCommand::About => DesignerTexts::APP_NAME,
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            MenuCommand::NewForm => "Ctrl+N",
            MenuCommand::OpenForm => "Ctrl+O",
            MenuCommand::SaveForm => "Ctrl+S",
            MenuCommand::SaveFormAs => "Ctrl+Shift+S",
            MenuCommand::GenerateCode => "Ctrl+Shift+B",
            MenuCommand::Quit => "Ctrl+Q",
            MenuCommand::Undo => "Ctrl+Z",
            MenuCommand::Redo => "Ctrl+Y",
            MenuCommand::Delete => "Del",
            MenuCommand::RunPreview => "F5",
            MenuCommand::ToggleTheme | MenuCommand::About => "",
        }
    }
}

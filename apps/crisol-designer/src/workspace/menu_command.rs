use crisol_engine::arranging::Alignment;

use crate::designer_texts::DesignerTexts;

/// Everything the menu bar and the command palette can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommand {
    NewProject,
    AddForm,
    SaveAll,
    ViewCode,
    ViewDesigner,
    BuildProject,
    RunProject,
    NewForm,
    OpenForm,
    SaveForm,
    SaveFormAs,
    CloseTab,
    GenerateCode,
    Quit,
    Undo,
    Redo,
    Copy,
    Paste,
    Duplicate,
    Delete,
    SelectParent,
    ToggleTheme,
    ShowStart,
    Arrange(Alignment),
    RunPreview,
    About,
}

impl MenuCommand {
    /// Every command, in menu order, for the command palette.
    pub const ALL: [MenuCommand; 32] = [
        MenuCommand::NewProject,
        MenuCommand::AddForm,
        MenuCommand::SaveAll,
        MenuCommand::ViewCode,
        MenuCommand::ViewDesigner,
        MenuCommand::BuildProject,
        MenuCommand::RunProject,
        MenuCommand::NewForm,
        MenuCommand::OpenForm,
        MenuCommand::SaveForm,
        MenuCommand::SaveFormAs,
        MenuCommand::CloseTab,
        MenuCommand::GenerateCode,
        MenuCommand::Quit,
        MenuCommand::Undo,
        MenuCommand::Redo,
        MenuCommand::Copy,
        MenuCommand::Paste,
        MenuCommand::Duplicate,
        MenuCommand::Delete,
        MenuCommand::SelectParent,
        MenuCommand::ToggleTheme,
        MenuCommand::ShowStart,
        MenuCommand::Arrange(Alignment::Left),
        MenuCommand::Arrange(Alignment::Center),
        MenuCommand::Arrange(Alignment::Right),
        MenuCommand::Arrange(Alignment::Top),
        MenuCommand::Arrange(Alignment::Bottom),
        MenuCommand::Arrange(Alignment::SameSize),
        MenuCommand::Arrange(Alignment::SpaceHorizontally),
        MenuCommand::RunPreview,
        MenuCommand::About,
    ];

    /// The commands of each menu, in the order of `DesignerTexts::MENUS`.
    pub fn items_for(menu: usize) -> &'static [MenuCommand] {
        match menu {
            0 => &[
                MenuCommand::NewProject,
                MenuCommand::OpenForm,
                MenuCommand::NewForm,
                MenuCommand::SaveForm,
                MenuCommand::SaveFormAs,
                MenuCommand::SaveAll,
                MenuCommand::CloseTab,
                MenuCommand::Quit,
            ],
            1 => &[
                MenuCommand::Undo,
                MenuCommand::Redo,
                MenuCommand::Copy,
                MenuCommand::Paste,
                MenuCommand::Duplicate,
                MenuCommand::Delete,
                MenuCommand::SelectParent,
            ],
            2 => &[
                MenuCommand::ViewCode,
                MenuCommand::ViewDesigner,
                MenuCommand::ShowStart,
                MenuCommand::ToggleTheme,
            ],
            3 => &[
                MenuCommand::Arrange(Alignment::Left),
                MenuCommand::Arrange(Alignment::Center),
                MenuCommand::Arrange(Alignment::Right),
                MenuCommand::Arrange(Alignment::Top),
                MenuCommand::Arrange(Alignment::Bottom),
                MenuCommand::Arrange(Alignment::SameSize),
                MenuCommand::Arrange(Alignment::SpaceHorizontally),
            ],
            4 => &[
                MenuCommand::AddForm,
                MenuCommand::GenerateCode,
                MenuCommand::BuildProject,
            ],
            5 => &[MenuCommand::RunPreview, MenuCommand::RunProject],
            6 => &[MenuCommand::About],
            _ => &[],
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MenuCommand::NewProject => "New project…",
            MenuCommand::AddForm => "Add form",
            MenuCommand::SaveAll => "Save all",
            MenuCommand::ViewCode => "Code",
            MenuCommand::ViewDesigner => "Designer",
            MenuCommand::BuildProject => "Build project",
            MenuCommand::RunProject => "Run project",
            MenuCommand::NewForm => "New form (outside a project)",
            MenuCommand::OpenForm => "Open project or form…",
            MenuCommand::SaveForm => "Save",
            MenuCommand::SaveFormAs => "Save as…",
            MenuCommand::CloseTab => "Close form",
            MenuCommand::GenerateCode => "Generate code",
            MenuCommand::Quit => "Exit",
            MenuCommand::Undo => "Undo",
            MenuCommand::Redo => "Redo",
            MenuCommand::Copy => "Copy",
            MenuCommand::Paste => "Paste",
            MenuCommand::Duplicate => "Duplicate",
            MenuCommand::Delete => "Delete",
            MenuCommand::SelectParent => "Select container",
            MenuCommand::ToggleTheme => "Light / dark theme",
            MenuCommand::ShowStart => "Start page",
            MenuCommand::Arrange(alignment) => alignment.label(),
            MenuCommand::RunPreview => "Run preview",
            MenuCommand::About => DesignerTexts::ABOUT,
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            MenuCommand::NewProject => "Ctrl+Shift+N",
            MenuCommand::AddForm => "Ctrl+Shift+A",
            MenuCommand::SaveAll => "Ctrl+Shift+S",
            MenuCommand::ViewCode => "F7",
            MenuCommand::ViewDesigner => "Shift+F7",
            MenuCommand::BuildProject => "Ctrl+Shift+B",
            MenuCommand::RunProject => "Ctrl+F5",
            MenuCommand::NewForm => "Ctrl+N",
            MenuCommand::OpenForm => "Ctrl+O",
            MenuCommand::SaveForm => "Ctrl+S",
            MenuCommand::CloseTab => "Ctrl+W",
            MenuCommand::Quit => "Ctrl+Q",
            MenuCommand::Undo => "Ctrl+Z",
            MenuCommand::Redo => "Ctrl+Y",
            MenuCommand::Copy => "Ctrl+C",
            MenuCommand::Paste => "Ctrl+V",
            MenuCommand::Duplicate => "Ctrl+D",
            MenuCommand::Delete => "Del",
            MenuCommand::SelectParent => "Esc",
            MenuCommand::RunPreview => "F5",
            MenuCommand::SaveFormAs
            | MenuCommand::GenerateCode
            | MenuCommand::ToggleTheme
            | MenuCommand::ShowStart
            | MenuCommand::Arrange(_)
            | MenuCommand::About => "",
        }
    }
}

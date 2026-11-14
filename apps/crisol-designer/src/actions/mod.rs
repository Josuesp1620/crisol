//! Designer commands the menus, the tool bar and the keyboard share.

mod designer_actions;
mod designer_key_bindings;

pub use designer_actions::{
    CloseMenu, DeleteSelection, GenerateCode, NewForm, OpenForm, Quit, Redo, RunPreview, SaveForm,
    SaveFormAs, ToggleTheme, Undo,
};
pub use designer_key_bindings::DesignerKeyBindings;

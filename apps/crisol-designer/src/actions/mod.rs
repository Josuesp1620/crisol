//! Designer commands the menus, the tool bar and the keyboard share.

mod designer_actions;
mod designer_key_bindings;

pub use designer_actions::{
    AddForm, BuildProject, CloseMenu, CloseTab, CopySelection, DeleteSelection, DuplicateSelection,
    GenerateCode, NewForm, NewProject, NudgeDown, NudgeDownFar, NudgeLeft, NudgeLeftFar,
    NudgeRight, NudgeRightFar, NudgeUp, NudgeUpFar, OpenForm, OpenPalette, OpenProject,
    PasteClipboard, Quit, Redo, RunPreview, RunProject, SaveAll, SaveForm, SaveFormAs,
    SelectParent, ToggleTheme, Undo, ViewCode, ViewDesigner,
};
pub use designer_key_bindings::DesignerKeyBindings;

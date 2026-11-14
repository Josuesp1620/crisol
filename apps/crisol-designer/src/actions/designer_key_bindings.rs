use gpui::{App, KeyBinding};

use super::{
    CloseMenu, DeleteSelection, GenerateCode, NewForm, OpenForm, Quit, Redo, RunPreview, SaveForm,
    SaveFormAs, Undo,
};

/// The designer's keyboard shortcuts.
pub struct DesignerKeyBindings;

impl DesignerKeyBindings {
    pub const CONTEXT: &'static str = "CrisolDesigner";

    pub fn install(cx: &mut App) {
        let context = Some(Self::CONTEXT);
        cx.bind_keys([
            KeyBinding::new("ctrl-n", NewForm, context),
            KeyBinding::new("ctrl-o", OpenForm, context),
            KeyBinding::new("ctrl-s", SaveForm, context),
            KeyBinding::new("ctrl-shift-s", SaveFormAs, context),
            KeyBinding::new("ctrl-shift-b", GenerateCode, context),
            KeyBinding::new("ctrl-z", Undo, context),
            KeyBinding::new("ctrl-y", Redo, context),
            KeyBinding::new("ctrl-shift-z", Redo, context),
            KeyBinding::new("delete", DeleteSelection, context),
            KeyBinding::new("f5", RunPreview, context),
            KeyBinding::new("escape", CloseMenu, context),
            KeyBinding::new("ctrl-q", Quit, context),
        ]);
    }
}

use gpui::{App, KeyBinding};

use super::{
    AddForm, BuildProject, CloseMenu, CloseTab, CopySelection, DeleteSelection, DuplicateSelection,
    NewForm, NewProject, NudgeDown, NudgeDownFar, NudgeLeft, NudgeLeftFar, NudgeRight,
    NudgeRightFar, NudgeUp, NudgeUpFar, OpenForm, OpenPalette, OpenProject, PasteClipboard, Quit,
    Redo, RunPreview, RunProject, SaveAll, SaveForm, Undo, ViewCode, ViewDesigner,
};

/// The designer's keyboard shortcuts. Text fields have their own context, so arrows and
/// clipboard keys keep working inside them.
pub struct DesignerKeyBindings;

impl DesignerKeyBindings {
    pub const CONTEXT: &'static str = "CrisolDesigner";

    pub fn install(cx: &mut App) {
        let context = Some(Self::CONTEXT);
        cx.bind_keys([
            KeyBinding::new("ctrl-shift-n", NewProject, context),
            KeyBinding::new("ctrl-n", NewForm, context),
            KeyBinding::new("ctrl-o", OpenForm, context),
            KeyBinding::new("ctrl-s", SaveForm, context),
            KeyBinding::new("ctrl-shift-o", OpenProject, context),
            KeyBinding::new("ctrl-shift-a", AddForm, context),
            KeyBinding::new("ctrl-shift-s", SaveAll, context),
            KeyBinding::new("f7", ViewCode, context),
            KeyBinding::new("shift-f7", ViewDesigner, context),
            KeyBinding::new("ctrl-f5", RunProject, context),
            KeyBinding::new("ctrl-w", CloseTab, context),
            KeyBinding::new("ctrl-shift-b", BuildProject, context),
            KeyBinding::new("ctrl-z", Undo, context),
            KeyBinding::new("ctrl-y", Redo, context),
            KeyBinding::new("ctrl-shift-z", Redo, context),
            KeyBinding::new("delete", DeleteSelection, context),
            KeyBinding::new("ctrl-c", CopySelection, context),
            KeyBinding::new("ctrl-v", PasteClipboard, context),
            KeyBinding::new("ctrl-d", DuplicateSelection, context),
            KeyBinding::new("left", NudgeLeft, context),
            KeyBinding::new("right", NudgeRight, context),
            KeyBinding::new("up", NudgeUp, context),
            KeyBinding::new("down", NudgeDown, context),
            KeyBinding::new("shift-left", NudgeLeftFar, context),
            KeyBinding::new("shift-right", NudgeRightFar, context),
            KeyBinding::new("shift-up", NudgeUpFar, context),
            KeyBinding::new("shift-down", NudgeDownFar, context),
            KeyBinding::new("f5", RunPreview, context),
            KeyBinding::new("ctrl-shift-p", OpenPalette, context),
            KeyBinding::new("escape", CloseMenu, context),
            KeyBinding::new("ctrl-q", Quit, context),
        ]);
    }
}

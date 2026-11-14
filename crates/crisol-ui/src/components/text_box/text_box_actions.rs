use gpui::{App, KeyBinding, actions};

actions!(
    crisol_text_box,
    [
        Backspace,
        Delete,
        MoveLeft,
        MoveRight,
        SelectLeft,
        SelectRight,
        SelectAll,
        MoveHome,
        MoveEnd,
        SelectHome,
        SelectEnd,
        Paste,
        Cut,
        Copy,
        Submit,
    ]
);

/// The keyboard shortcuts of every text box.
pub struct TextBoxKeyBindings;

impl TextBoxKeyBindings {
    pub const CONTEXT: &'static str = "CrisolTextBox";

    pub fn install(cx: &mut App) {
        let context = Some(Self::CONTEXT);
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, context),
            KeyBinding::new("delete", Delete, context),
            KeyBinding::new("left", MoveLeft, context),
            KeyBinding::new("right", MoveRight, context),
            KeyBinding::new("shift-left", SelectLeft, context),
            KeyBinding::new("shift-right", SelectRight, context),
            KeyBinding::new("home", MoveHome, context),
            KeyBinding::new("end", MoveEnd, context),
            KeyBinding::new("shift-home", SelectHome, context),
            KeyBinding::new("shift-end", SelectEnd, context),
            KeyBinding::new("enter", Submit, context),
            KeyBinding::new("ctrl-a", SelectAll, context),
            KeyBinding::new("ctrl-v", Paste, context),
            KeyBinding::new("ctrl-x", Cut, context),
            KeyBinding::new("ctrl-c", Copy, context),
            KeyBinding::new("cmd-a", SelectAll, context),
            KeyBinding::new("cmd-v", Paste, context),
            KeyBinding::new("cmd-x", Cut, context),
            KeyBinding::new("cmd-c", Copy, context),
        ]);
    }
}

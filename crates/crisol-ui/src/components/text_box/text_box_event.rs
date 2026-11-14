/// Events a text box raises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBoxEvent {
    /// The text changed by typing, pasting or deleting.
    Changed,
    /// The user pressed Enter.
    Submitted,
    FocusGained,
    FocusLost,
}

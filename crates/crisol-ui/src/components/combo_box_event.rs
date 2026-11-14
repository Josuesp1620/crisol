/// Events a combo box raises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboBoxEvent {
    /// The user picked another option.
    SelectionChanged,
    /// The list opened.
    Opened,
    FocusGained,
    FocusLost,
}

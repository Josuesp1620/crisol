/// What a text box holds; it decides which characters are accepted and how text shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputKind {
    #[default]
    Text,
    Email,
    /// Digits only, with an optional leading minus sign and one decimal point.
    Number,
    /// Shown as dots.
    Password,
    /// Accepts line breaks typed with Shift+Enter; shown on one line for now.
    Multiline,
}

impl InputKind {
    pub fn accepts(self, character: char) -> bool {
        match self {
            InputKind::Number => character.is_ascii_digit() || character == '-' || character == '.',
            InputKind::Multiline => true,
            _ => character != '\n' && character != '\r',
        }
    }

    pub fn masks_text(self) -> bool {
        self == InputKind::Password
    }
}

/// Controls that keep state between frames (typed text, checked box) and therefore
/// become `Entity` fields of the generated struct.
pub struct StatefulKind;

impl StatefulKind {
    const KINDS: [&'static str; 4] = ["TextBox", "CheckBox", "RadioButton", "ComboBox"];

    pub fn is_stateful(kind: &str) -> bool {
        Self::KINDS.contains(&kind)
    }

    /// The event enum a stateful control emits (`TextBoxEvent`).
    pub fn event_type(kind: &str) -> String {
        format!("{kind}Event")
    }
}

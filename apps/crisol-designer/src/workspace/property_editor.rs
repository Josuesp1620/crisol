use gpui::Subscription;

use super::{EditorField, PropertyKey};

/// A property row being edited, with the field that edits it. Dropping the editor ends
/// the subscription to the field.
pub struct PropertyEditor {
    pub key: PropertyKey,
    pub field: EditorField,
    pub _subscription: Subscription,
}

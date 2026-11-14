use gpui::Subscription;

use super::{EditorField, PropertyKey};

/// A property row being edited, with the field that edits it. Dropping the editor ends
/// the subscription to the field.
pub struct PropertyEditor {
    pub key: PropertyKey,
    pub field: EditorField,
    _subscription: Subscription,
}

impl PropertyEditor {
    pub fn new(key: PropertyKey, field: EditorField, subscription: Subscription) -> Self {
        Self {
            key,
            field,
            _subscription: subscription,
        }
    }
}

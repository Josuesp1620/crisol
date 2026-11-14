use crisol_ui::components::{ComboBox, TextBox};
use gpui::Entity;

/// The control used to edit a property value in place.
pub enum EditorField {
    Text(Entity<TextBox>),
    Choice(Entity<ComboBox>),
}

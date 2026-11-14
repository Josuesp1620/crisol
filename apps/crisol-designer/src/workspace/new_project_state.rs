use crisol_ui::components::TextBox;
use crisol_ui::theme::ThemeMode;
use gpui::Entity;

use crate::project::ProjectTemplate;

/// The New project dialog while it is open.
pub struct NewProjectState {
    pub template: ProjectTemplate,
    pub name: Entity<TextBox>,
    pub location: Entity<TextBox>,
    pub theme: ThemeMode,
    pub error: Option<String>,
}

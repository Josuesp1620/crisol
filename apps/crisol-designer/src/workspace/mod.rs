//! The designer window: its state and the behavior behind every panel.

mod bottom_tab;
mod canvas_interaction;
mod designer_workspace;
mod editor_field;
mod explorer_entry;
mod menu_command;
mod new_project_state;
mod property_editor;
mod property_key;
mod property_tab;
mod toolbox_drag;
mod workspace_actions;
mod workspace_arranging;
mod workspace_canvas;
mod workspace_code;
mod workspace_editing;
mod workspace_files;
mod workspace_menus;
mod workspace_projects;
mod workspace_properties;
mod workspace_views;

pub use bottom_tab::BottomTab;
pub use canvas_interaction::CanvasInteraction;
pub use designer_workspace::DesignerWorkspace;
pub use editor_field::EditorField;
pub use explorer_entry::ExplorerEntry;
pub use menu_command::MenuCommand;
pub use new_project_state::NewProjectState;
pub use property_editor::PropertyEditor;
pub use property_key::PropertyKey;
pub use property_tab::PropertyTab;
pub use toolbox_drag::{ToolboxDrag, ToolboxDragPreview};

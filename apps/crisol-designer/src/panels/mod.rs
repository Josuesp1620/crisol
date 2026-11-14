//! The regions of the designer window, each drawn from the workspace state.

mod bottom_panel;
mod code_panel;
mod document_area;
mod explorer_panel;
mod menu_dropdown;
mod new_project_dialog;
mod property_grid_panel;
mod property_row;
mod property_rows_builder;
mod side_panel;
mod start_page;
mod status_bar;
mod title_bar;
mod tool_bar;
mod toolbox_panel;

pub use bottom_panel::BottomPanel;
pub use code_panel::CodePanel;
pub use document_area::DocumentArea;
pub use explorer_panel::ExplorerPanel;
pub use menu_dropdown::MenuDropdown;
pub use new_project_dialog::NewProjectDialog;
pub use property_grid_panel::PropertyGridPanel;
pub use property_row::PropertyRow;
pub use property_rows_builder::PropertyRowsBuilder;
pub use side_panel::SidePanel;
pub use start_page::StartPage;
pub use status_bar::StatusBar;
pub use title_bar::TitleBar;
pub use tool_bar::ToolBar;
pub use toolbox_panel::ToolboxPanel;

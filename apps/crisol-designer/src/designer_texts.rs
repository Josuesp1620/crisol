/// Every text the designer shows, in one place.
pub struct DesignerTexts;

impl DesignerTexts {
    pub const APP_NAME: &'static str = "Crisol";
    pub const MENUS: [&'static str; 7] =
        ["File", "Edit", "View", "Format", "Project", "Run", "Help"];
    pub const COMMAND_SEARCH: &'static str = "Search commands or controls";
    pub const COMMAND_SEARCH_SHORTCUT: &'static str = "Ctrl+Shift+P";
    pub const ALIGN: &'static str = "Align";
    pub const PROFILE: &'static str = "Profile";
    pub const PROFILE_DESKTOP: &'static str = "Desktop";
    pub const PROFILE_WEB: &'static str = "Web";
    pub const PROFILE_PHONE: &'static str = "Phone";
    pub const PROFILE_ALL: &'static str = "All";
    pub const ZOOM: &'static str = "100 %";
    pub const VALID: &'static str = "Valid";
    pub const RUN: &'static str = "Run";
    pub const RUN_SHORTCUT: &'static str = "F5";
    pub const TOOLBOX: &'static str = "Toolbox";
    pub const SEARCH_CONTROL: &'static str = "Search control";
    pub const POINTER: &'static str = "Pointer";
    pub const DESKTOP_ONLY: &'static str = "desktop only";
    pub const LATER_PHASE: &'static str = "phase 4";
    pub const LATER_GROUPS: [&'static str; 3] = ["Data", "Menus and toolbars", "Dialogs"];
    pub const TOOLBOX_HINT: &'static str =
        "Drag a control onto the form, or double-click it to add it to the selected container.";
    pub const EXPLORER: &'static str = "Project explorer";
    pub const PROPERTIES: &'static str = "Properties";
    pub const SEARCH: &'static str = "Search";
    pub const NAME_PROPERTY: &'static str = "(Name)";
    pub const NAME_DESCRIPTION: &'static str =
        "The name of the control in code. It must be unique in the form and written in snake_case.";
    pub const DESIGN_CATEGORY: &'static str = "Design";
    pub const PLACEMENT_PROPERTY: &'static str = "Placement";
    pub const PLACEMENT_DESCRIPTION: &'static str =
        "Where the control sits in its container. Drag it on the canvas to change it.";
    pub const FORM_TITLE: &'static str = "Title";
    pub const FORM_TITLE_DESCRIPTION: &'static str = "The text in the title bar of the window.";
    pub const FORM_TYPE: &'static str = "Type name";
    pub const FORM_TYPE_DESCRIPTION: &'static str =
        "The Rust type generated for the form, in PascalCase.";
    pub const FORM_WIDTH: &'static str = "Width";
    pub const FORM_HEIGHT: &'static str = "Height";
    pub const FORM_SIZE_DESCRIPTION: &'static str =
        "The size the window opens with, in logical pixels.";
    pub const EVENTS_HINT: &'static str = "Click an empty event to connect it to a method. The method is added to your file when you generate code; existing code is never removed.";
    pub const NO_SELECTION: &'static str = "Select a control to see its properties.";
    pub const ERROR_LIST: &'static str = "Error list";
    pub const OUTPUT: &'static str = "Output";
    pub const HISTORY: &'static str = "History";
    pub const ERRORS: &'static str = "errors";
    pub const WARNINGS: &'static str = "warnings";
    pub const MESSAGES: &'static str = "messages";
    pub const NOTHING_TO_SHOW: &'static str = "Nothing here yet.";
    pub const READY: &'static str = "Ready";
    pub const UNSAVED_DOCUMENT: &'static str = "untitled.crisol";
    pub const DESIGN_TAB_SUFFIX: &'static str = "[Design]";
    pub const CANVAS_INFO: &'static str = "Desktop";
    pub const GRID_INFO: &'static str = "grid 8 px";
    pub const PREVIEW_TITLE: &'static str = "Preview";
    pub const RENDERER: &'static str = if cfg!(target_os = "windows") {
        "DirectX 11"
    } else if cfg!(target_os = "macos") {
        "Metal"
    } else {
        "wgpu"
    };
    pub const GPUI_REVISION: &'static str = "GPUI 56a157e5";
}

use gpui::{Pixels, px};

/// The single place where the designer's layout values live.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesignerSettings {
    pub window_width: Pixels,
    pub window_height: Pixels,
    pub title_bar_height: Pixels,
    pub tool_bar_height: Pixels,
    pub status_bar_height: Pixels,
    pub toolbox_width: Pixels,
    pub side_panel_width: Pixels,
    pub explorer_height: Pixels,
    pub bottom_panel_height: Pixels,
    pub document_tabs_height: Pixels,
    pub breadcrumb_height: Pixels,
    pub pane_header_height: Pixels,
    pub row_height: Pixels,
    pub property_name_width: Pixels,
    pub description_height: Pixels,
    pub canvas_dot_spacing: Pixels,
    /// Where the form window sits inside the canvas.
    pub form_offset_x: Pixels,
    pub form_offset_y: Pixels,
    pub form_title_height: Pixels,
    pub handle_size: Pixels,
    pub menu_width: Pixels,
}

impl Default for DesignerSettings {
    fn default() -> Self {
        Self {
            window_width: px(1440.),
            window_height: px(900.),
            title_bar_height: px(36.),
            tool_bar_height: px(42.),
            status_bar_height: px(26.),
            toolbox_width: px(232.),
            side_panel_width: px(300.),
            explorer_height: px(236.),
            bottom_panel_height: px(150.),
            document_tabs_height: px(32.),
            breadcrumb_height: px(26.),
            pane_header_height: px(30.),
            row_height: px(26.),
            property_name_width: px(128.),
            description_height: px(84.),
            canvas_dot_spacing: px(16.),
            form_offset_x: px(70.),
            form_offset_y: px(34.),
            form_title_height: px(34.),
            handle_size: px(7.),
            menu_width: px(240.),
        }
    }
}

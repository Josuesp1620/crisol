use gpui::{Pixels, px};

/// Sizes and radii shared by every control.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeMetrics {
    pub text_size: Pixels,
    pub small_text_size: Pixels,
    pub tiny_text_size: Pixels,
    pub control_height: Pixels,
    pub button_height: Pixels,
    pub button_min_width: Pixels,
    pub control_radius: Pixels,
    pub small_radius: Pixels,
    pub panel_radius: Pixels,
    pub control_padding: Pixels,
    pub check_size: Pixels,
    pub icon_size: Pixels,
}

impl Default for ThemeMetrics {
    fn default() -> Self {
        Self {
            text_size: px(12.),
            small_text_size: px(11.),
            tiny_text_size: px(10.5),
            control_height: px(30.),
            button_height: px(32.),
            button_min_width: px(100.),
            control_radius: px(6.),
            small_radius: px(5.),
            panel_radius: px(8.),
            control_padding: px(10.),
            check_size: px(16.),
            icon_size: px(15.),
        }
    }
}

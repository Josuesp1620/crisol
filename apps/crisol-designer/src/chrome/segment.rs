use crisol_ui::icon::IconName;
use gpui::SharedString;

/// One option of a segmented control.
pub struct Segment {
    pub icon: IconName,
    pub label: SharedString,
    pub active: bool,
}

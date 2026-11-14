use crisol_ui::icon::IconName;
use gpui::SharedString;

/// One option of a segmented control.
pub struct Segment {
    pub icon: IconName,
    pub label: SharedString,
    pub active: bool,
    /// False for options of a later phase: shown dimmed and not clickable.
    pub available: bool,
}

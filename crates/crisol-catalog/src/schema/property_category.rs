/// The groups of the property grid, in the order they are shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PropertyCategory {
    Appearance,
    Behavior,
    Layout,
    Accessibility,
    Data,
}

impl PropertyCategory {
    pub const ORDER: [PropertyCategory; 5] = [
        PropertyCategory::Appearance,
        PropertyCategory::Behavior,
        PropertyCategory::Layout,
        PropertyCategory::Accessibility,
        PropertyCategory::Data,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PropertyCategory::Appearance => "Appearance",
            PropertyCategory::Behavior => "Behavior",
            PropertyCategory::Layout => "Layout",
            PropertyCategory::Accessibility => "Accessibility",
            PropertyCategory::Data => "Data",
        }
    }
}

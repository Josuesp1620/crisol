/// The groups of the toolbox, in the order they are shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolboxGroup {
    CommonControls,
    Containers,
    /// Not offered in the toolbox (the form itself).
    Hidden,
}

impl ToolboxGroup {
    pub const VISIBLE: [ToolboxGroup; 2] = [ToolboxGroup::CommonControls, ToolboxGroup::Containers];

    pub fn label(self) -> &'static str {
        match self {
            ToolboxGroup::CommonControls => "Common controls",
            ToolboxGroup::Containers => "Containers",
            ToolboxGroup::Hidden => "",
        }
    }
}

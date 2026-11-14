/// How several selected controls are lined up with the primary one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Top,
    Bottom,
    /// Same width and height as the primary control.
    SameSize,
    /// Equal gaps between the controls, from the leftmost to the rightmost.
    SpaceHorizontally,
}

impl Alignment {
    pub fn label(self) -> &'static str {
        match self {
            Alignment::Left => "Align left edges",
            Alignment::Center => "Align centers",
            Alignment::Right => "Align right edges",
            Alignment::Top => "Align tops",
            Alignment::Bottom => "Align bottoms",
            Alignment::SameSize => "Make same size",
            Alignment::SpaceHorizontally => "Space evenly",
        }
    }
}

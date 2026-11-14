/// What the property grid lists: properties or events (the lightning button).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyTab {
    #[default]
    Properties,
    Events,
}

/// Whether a control holds children, and how it places them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRule {
    /// Holds no children.
    Leaf,
    /// Children one after another, vertically or horizontally.
    Flow,
    /// Children in rows and columns.
    Grid,
    /// Children at fixed positions with anchors; desktop only.
    Absolute,
}

impl ContainerRule {
    pub fn accepts_children(self) -> bool {
        self != ContainerRule::Leaf
    }
}

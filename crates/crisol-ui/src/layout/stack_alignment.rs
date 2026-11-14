/// Where the children of a stack sit along its direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackAlignment {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
}

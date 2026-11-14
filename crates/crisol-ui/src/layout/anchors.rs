/// The edges of an absolute panel a control keeps its distance to, as in Windows Forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "one flag per edge, as in Windows Forms"
)]
pub struct Anchors {
    pub top: bool,
    pub left: bool,
    pub right: bool,
    pub bottom: bool,
}

impl Anchors {
    #[allow(
        clippy::fn_params_excessive_bools,
        reason = "one flag per edge, as in Windows Forms"
    )]
    pub fn new(top: bool, left: bool, right: bool, bottom: bool) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}

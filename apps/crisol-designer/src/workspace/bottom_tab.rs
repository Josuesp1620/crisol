/// The tabs of the panel under the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BottomTab {
    #[default]
    Errors,
    Output,
    History,
}

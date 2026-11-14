/// The single place where the editor's tunable values live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineSettings {
    /// A dragged edge closer than this to a guide jumps onto it, in logical pixels.
    pub snap_distance: i32,
    /// Side of the square resize handles drawn around the selection.
    pub handle_size: i32,
    /// Smallest width or height a resize can leave.
    pub minimum_control_size: i32,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            snap_distance: 6,
            handle_size: 7,
            minimum_control_size: 8,
        }
    }
}

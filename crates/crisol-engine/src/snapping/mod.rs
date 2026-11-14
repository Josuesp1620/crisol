//! Smart guides: edges and centers of a dragged control jump onto those of its neighbours.

mod guide_axis;
mod snap_engine;
mod snap_guide;
mod snap_result;

pub use guide_axis::GuideAxis;
pub use snap_engine::SnapEngine;
pub use snap_guide::SnapGuide;
pub use snap_result::SnapResult;

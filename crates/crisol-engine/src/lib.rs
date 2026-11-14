//! Designer logic without a user interface: what is under the pointer, what is selected,
//! where a drag lands and which guides it snaps to. The designer only draws the results.

pub mod arranging;
pub mod dragging;
pub mod engine_settings;
pub mod layout;
pub mod selection;
pub mod snapping;

pub use engine_settings::EngineSettings;

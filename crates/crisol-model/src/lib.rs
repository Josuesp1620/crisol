//! The Crisol form document: a tree of controls with their properties, placement and events,
//! and its canonical `.crisol` file format. Nothing here depends on a user interface.

pub mod document;
pub mod error;
pub mod events;
pub mod file;
pub mod format_settings;
pub mod geometry;
pub mod identity;
pub mod layout;
pub mod project;
pub mod properties;

pub use error::{ModelError, ModelResult};
pub use format_settings::FormatSettings;

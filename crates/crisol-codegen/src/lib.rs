//! Code generation from `.crisol` documents. The first target is Rust for GPUI using the
//! `crisol-ui` components; the user's handlers live in their own file, never rewritten.

pub mod code_generator;
pub mod codegen_settings;
pub mod error;
pub mod generated_file;
pub mod rust_gpui;
pub mod stubs;
pub mod writing;

pub use code_generator::CodeGenerator;
pub use codegen_settings::CodegenSettings;
pub use error::{CodegenError, CodegenResult};
pub use generated_file::GeneratedFile;

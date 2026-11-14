//! Rust code for GPUI built on the `crisol-ui` components.

mod constructor_writer;
mod element_writer;
mod event_binding;
mod event_binding_collector;
mod events_trait_writer;
mod generation_context;
mod rust_gpui_generator;
mod stateful_kind;

pub use event_binding::EventBinding;
pub use event_binding_collector::EventBindingCollector;
pub use rust_gpui_generator::RustGpuiGenerator;

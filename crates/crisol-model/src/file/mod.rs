//! The `.crisol` file: canonical JSON with the form and its tree of nodes nested in order.

mod document_file;
mod document_serializer;
mod node_record;

pub use document_serializer::DocumentSerializer;

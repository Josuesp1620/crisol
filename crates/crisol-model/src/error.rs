use thiserror::Error;

use crate::identity::NodeId;

/// Every failure the document model can report.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("invalid .crisol content: {0}")]
    InvalidFile(#[from] serde_json::Error),

    #[error("unsupported format {found}; this version reads {expected}")]
    UnsupportedFormat { found: String, expected: String },

    #[error("{0:?} is not a valid name: use snake_case letters, digits and underscores")]
    InvalidIdentifier(String),

    #[error("{0:?} is not a valid type name: use PascalCase letters and digits")]
    InvalidTypeName(String),

    #[error("no node with id {0}")]
    NodeNotFound(NodeId),

    #[error("the root of the form cannot be removed or moved")]
    RootIsFixed,

    #[error("a node cannot be moved inside itself or its own children")]
    CyclicMove,

    #[error("the name {0} is already used in this form")]
    DuplicateName(String),

    #[error("position {index} is outside the {length} children of the container")]
    ChildIndexOutOfRange { index: usize, length: usize },
}

pub type ModelResult<T> = Result<T, ModelError>;

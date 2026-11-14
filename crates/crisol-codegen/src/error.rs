use thiserror::Error;

use crisol_model::ModelError;

/// Every failure code generation can report.
#[derive(Debug, Error)]
pub enum CodegenError {
    #[error(transparent)]
    Model(#[from] ModelError),

    #[error("the form has {0} error(s); fix them before generating code")]
    InvalidDocument(usize),

    #[error("no `impl {0} for` block was found in the handlers file")]
    MissingHandlersImpl(String),
}

pub type CodegenResult<T> = Result<T, CodegenError>;

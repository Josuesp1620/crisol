use thiserror::Error;

use crisol_codegen::CodegenError;
use crisol_model::ModelError;

/// Every failure the command line reports.
#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0}")]
    Usage(String),

    #[error("cannot read or write {path}: {source}")]
    File {
        path: String,
        source: std::io::Error,
    },

    #[error(transparent)]
    Model(#[from] ModelError),

    #[error(transparent)]
    Codegen(#[from] CodegenError),
}

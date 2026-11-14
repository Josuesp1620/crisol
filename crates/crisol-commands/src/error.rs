use thiserror::Error;

use crisol_model::ModelError;

/// Every failure a command can report.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    Model(#[from] ModelError),

    #[error("nothing to undo")]
    NothingToUndo,

    #[error("nothing to redo")]
    NothingToRedo,
}

pub type CommandResult<T> = Result<T, CommandError>;

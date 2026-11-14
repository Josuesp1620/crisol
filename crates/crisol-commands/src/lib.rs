//! Every change to a document is a reversible command. The designer, the command line and the
//! assistant all go through the same commands, and each one is one step of undo.

pub mod command;
pub mod commands;
pub mod error;
pub mod history;

pub use command::Command;
pub use error::{CommandError, CommandResult};

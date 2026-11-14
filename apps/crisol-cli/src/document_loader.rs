use std::fs;
use std::path::Path;

use crisol_model::document::FormDocument;
use crisol_model::file::DocumentSerializer;

use crate::cli_error::CliError;

/// Reads `.crisol` files from disk.
pub struct DocumentLoader;

impl DocumentLoader {
    pub fn load(path: &Path) -> Result<FormDocument, CliError> {
        let text = fs::read_to_string(path).map_err(|source| CliError::File {
            path: path.display().to_string(),
            source,
        })?;
        Ok(DocumentSerializer::default().read(&text)?)
    }
}

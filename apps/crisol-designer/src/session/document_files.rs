use std::fs;
use std::path::Path;

use crisol_model::document::FormDocument;
use crisol_model::file::DocumentSerializer;

/// Reads and writes `.crisol` files.
pub struct DocumentFiles;

impl DocumentFiles {
    pub fn load(path: &Path) -> Result<FormDocument, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("Cannot read {}: {error}", path.display()))?;
        Self::parse(&text)
    }

    pub fn parse(text: &str) -> Result<FormDocument, String> {
        DocumentSerializer::default()
            .read(text)
            .map_err(|error| error.to_string())
    }

    pub fn save(document: &FormDocument, path: &Path) -> Result<(), String> {
        let text = DocumentSerializer::default()
            .write(document)
            .map_err(|error| error.to_string())?;
        fs::write(path, text).map_err(|error| format!("Cannot write {}: {error}", path.display()))
    }
}

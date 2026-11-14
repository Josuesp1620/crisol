use serde::{Deserialize, Serialize};

use super::ProjectManifest;
use crate::{ModelError, ModelResult};

/// Reads and writes `.crisolproj` text.
pub struct ProjectSerializer;

/// The file on disk: the format version, then the manifest.
#[derive(Serialize, Deserialize)]
struct ProjectFile {
    format: String,
    #[serde(flatten)]
    manifest: ProjectManifest,
}

impl ProjectSerializer {
    pub const FORMAT: &'static str = "crisol-project/1";
    pub const EXTENSION: &'static str = "crisolproj";

    pub fn write(manifest: &ProjectManifest) -> ModelResult<String> {
        let file = ProjectFile {
            format: Self::FORMAT.to_owned(),
            manifest: manifest.clone(),
        };
        let mut text = serde_json::to_string_pretty(&file)?;
        text.push('\n');
        Ok(text)
    }

    pub fn read(text: &str) -> ModelResult<ProjectManifest> {
        let file: ProjectFile = serde_json::from_str(text)?;
        if file.format != Self::FORMAT {
            return Err(ModelError::UnsupportedFormat {
                found: file.format,
                expected: Self::FORMAT.to_owned(),
            });
        }
        Ok(file.manifest)
    }
}

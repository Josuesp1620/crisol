use std::fs;
use std::path::{Path, PathBuf};

use crisol_model::project::{ProjectManifest, ProjectSerializer};

/// The open project: its file and its manifest.
#[derive(Debug, Clone)]
pub struct ProjectSession {
    manifest_path: PathBuf,
    manifest: ProjectManifest,
}

impl ProjectSession {
    pub fn load(manifest_path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(manifest_path)
            .map_err(|error| format!("Cannot read {}: {error}", manifest_path.display()))?;
        let manifest = ProjectSerializer::read(&text).map_err(|error| error.to_string())?;
        Ok(Self {
            manifest_path: manifest_path.to_path_buf(),
            manifest,
        })
    }

    pub fn create(manifest_path: PathBuf, manifest: ProjectManifest) -> Result<Self, String> {
        let session = Self {
            manifest_path,
            manifest,
        };
        session.save()?;
        Ok(session)
    }

    /// The project that holds `form`: the first `.crisolproj` found in its folder or above.
    pub fn find_for(form: &Path) -> Option<Self> {
        let mut folder = form.parent();
        while let Some(current) = folder {
            if let Ok(entries) = fs::read_dir(current) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path
                        .extension()
                        .is_some_and(|extension| extension == ProjectSerializer::EXTENSION)
                    {
                        return Self::load(&path).ok();
                    }
                }
            }
            folder = current.parent();
        }
        None
    }

    pub fn save(&self) -> Result<(), String> {
        let text = ProjectSerializer::write(&self.manifest).map_err(|error| error.to_string())?;
        fs::write(&self.manifest_path, text)
            .map_err(|error| format!("Cannot write {}: {error}", self.manifest_path.display()))
    }

    pub fn name(&self) -> &str {
        &self.manifest.name
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn root(&self) -> PathBuf {
        self.manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default()
    }

    /// Absolute paths of the project's forms.
    pub fn form_paths(&self) -> Vec<PathBuf> {
        let root = self.root();
        let mut paths = Vec::with_capacity(self.manifest.forms.len());
        for form in &self.manifest.forms {
            paths.push(root.join(form));
        }
        paths
    }

    pub fn startup_form(&self) -> Option<PathBuf> {
        self.manifest
            .startup_form
            .as_ref()
            .map(|form| self.root().join(form))
    }

    /// Adds a form saved inside the project folder and saves the manifest.
    pub fn add_form(&mut self, form: &Path) -> Result<(), String> {
        let relative = form
            .strip_prefix(self.root())
            .map_err(|_| format!("{} is outside the project folder.", form.display()))?;
        let mut parts = Vec::new();
        for component in relative.components() {
            parts.push(component.as_os_str().to_string_lossy().into_owned());
        }
        self.manifest.add_form(&parts.join("/"));
        self.save()
    }
}

use serde::{Deserialize, Serialize};

/// A project: its name and its forms, as paths relative to the project file with `/`
/// separators so the file is the same on every system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub forms: Vec<String>,
    /// The form the application opens first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_form: Option<String>,
}

impl ProjectManifest {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            forms: Vec::new(),
            startup_form: None,
        }
    }

    /// Adds a form once; the first form added becomes the startup form.
    pub fn add_form(&mut self, relative_path: &str) {
        if !self.forms.iter().any(|form| form == relative_path) {
            self.forms.push(relative_path.to_owned());
        }
        if self.startup_form.is_none() {
            self.startup_form = Some(relative_path.to_owned());
        }
    }

    pub fn remove_form(&mut self, relative_path: &str) {
        self.forms.retain(|form| form != relative_path);
        if self.startup_form.as_deref() == Some(relative_path) {
            self.startup_form = self.forms.first().cloned();
        }
    }
}

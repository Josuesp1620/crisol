use std::path::PathBuf;

use crisol_ui::icon::IconName;

/// One file of the project explorer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorerEntry {
    pub path: PathBuf,
    pub label: String,
    pub icon: IconName,
    pub depth: usize,
    /// Generated files are shown dimmed with a lock: Crisol rewrites them.
    pub generated: bool,
    pub is_form: bool,
}

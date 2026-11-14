use std::fs;
use std::path::Path;

use crate::session::CodeExport;

/// Rewrites a forms folder's `mod.rs` so it declares the module of every form in it.
pub struct FormsModuleWriter;

impl FormsModuleWriter {
    const HEADER: &'static str = "//! Forms designed in Crisol. Crisol rewrites this file and the `*.generated.rs` files;\n//! the `*_events.rs` files are yours.\n";

    pub fn write(forms_folder: &Path) -> Result<(), String> {
        let mut modules = Vec::new();
        let entries = fs::read_dir(forms_folder)
            .map_err(|error| format!("Cannot read {}: {error}", forms_folder.display()))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "crisol")
            {
                modules.push(CodeExport::module_of(&path));
            }
        }
        modules.sort();
        let mut text = String::from(Self::HEADER);
        for module in modules {
            text.push_str(&format!("\n#[rustfmt::skip]\n#[path = \"{module}.generated.rs\"]\npub mod {module};\nmod {module}_events;\n"));
        }
        let path = forms_folder.join("mod.rs");
        fs::write(&path, text).map_err(|error| format!("Cannot write {}: {error}", path.display()))
    }
}

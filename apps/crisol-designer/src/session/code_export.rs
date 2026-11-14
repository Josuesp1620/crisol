use std::fs;
use std::path::Path;

use crisol_catalog::WidgetCatalog;
use crisol_codegen::rust_gpui::{EventBindingCollector, RustGpuiGenerator};
use crisol_codegen::stubs::HandlerStubWriter;
use crisol_codegen::{CodeGenerator, CodegenSettings};
use crisol_model::document::FormDocument;

/// Writes the generated file and creates or extends the handlers file next to the form.
pub struct CodeExport<'catalog> {
    catalog: &'catalog WidgetCatalog,
}

impl<'catalog> CodeExport<'catalog> {
    pub fn new(catalog: &'catalog WidgetCatalog) -> Self {
        Self { catalog }
    }

    /// Returns the names of the files written.
    pub fn export(&self, document: &FormDocument, folder: &Path) -> Result<Vec<String>, String> {
        let generated = RustGpuiGenerator::new(self.catalog, CodegenSettings::default())
            .generate(document)
            .map_err(|error| error.to_string())?;
        Self::write(&folder.join(&generated.file_name), &generated.content)?;

        let bindings = EventBindingCollector::new(document, self.catalog).collect();
        let handlers = EventBindingCollector::unique_handlers(&bindings);
        let stubs = HandlerStubWriter::new(document, CodegenSettings::default());
        let handlers_path = folder.join(stubs.file_name());
        let handlers_text = if handlers_path.exists() {
            let existing = fs::read_to_string(&handlers_path).map_err(|error| error.to_string())?;
            stubs
                .extend(&existing, &handlers)
                .map_err(|error| error.to_string())?
        } else {
            stubs.create(&handlers)
        };
        Self::write(&handlers_path, &handlers_text)?;
        Ok(vec![generated.file_name, stubs.file_name()])
    }

    fn write(path: &Path, content: &str) -> Result<(), String> {
        fs::write(path, content)
            .map_err(|error| format!("Cannot write {}: {error}", path.display()))
    }
}

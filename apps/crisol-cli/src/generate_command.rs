use std::fs;
use std::path::Path;

use crisol_catalog::WidgetCatalog;
use crisol_codegen::rust_gpui::{EventBindingCollector, RustGpuiGenerator};
use crisol_codegen::stubs::HandlerStubWriter;
use crisol_codegen::{CodeGenerator, CodegenSettings};

use crate::cli_error::CliError;
use crate::document_loader::DocumentLoader;

/// Writes `<form>.generated.rs` and creates or extends `<form>_events.rs`.
pub struct GenerateCommand;

impl GenerateCommand {
    pub fn run(form: &Path, output: &Path) -> Result<(), CliError> {
        let document = DocumentLoader::load(form)?;
        let catalog = WidgetCatalog::standard();
        let module = Self::module_of(form);
        let generator = RustGpuiGenerator::new(&catalog, CodegenSettings::default());
        let mut generated = generator.generate(&document)?;
        generated.file_name = generator.file_name_for(&module);
        Self::write(&output.join(&generated.file_name), &generated.content)?;

        let bindings = EventBindingCollector::new(&document, &catalog).collect();
        let handlers = EventBindingCollector::unique_handlers(&bindings);
        let stubs =
            HandlerStubWriter::new(&document, CodegenSettings::default()).with_module(&module);
        let handlers_path = output.join(stubs.file_name());
        let handlers_text = if handlers_path.exists() {
            let existing = fs::read_to_string(&handlers_path).map_err(|source| CliError::File {
                path: handlers_path.display().to_string(),
                source,
            })?;
            stubs.extend(&existing, &handlers)?
        } else {
            stubs.create(&handlers)
        };
        Self::write(&handlers_path, &handlers_text)?;
        println!("Wrote {} and {}", generated.file_name, stubs.file_name());
        Ok(())
    }

    /// Modules are named after the form file: `customer.crisol` gives `customer`.
    fn module_of(form: &Path) -> String {
        form.file_stem()
            .map(|stem| stem.to_string_lossy().replace('-', "_"))
            .unwrap_or_default()
    }

    fn write(path: &Path, content: &str) -> Result<(), CliError> {
        fs::write(path, content).map_err(|source| CliError::File {
            path: path.display().to_string(),
            source,
        })
    }
}

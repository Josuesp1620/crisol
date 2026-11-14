use std::path::Path;

use crisol_catalog::WidgetCatalog;
use crisol_catalog::validation::{CatalogValidator, DiagnosticSeverity};

use crate::cli_error::CliError;
use crate::document_loader::DocumentLoader;

/// Validates a form and prints what the designer's error list would show.
pub struct CheckCommand;

impl CheckCommand {
    /// Returns whether the form has no errors.
    pub fn run(form: &Path) -> Result<bool, CliError> {
        let document = DocumentLoader::load(form)?;
        let catalog = WidgetCatalog::standard();
        let diagnostics = CatalogValidator::new(&catalog).validate(&document);
        let mut errors = 0;
        for diagnostic in &diagnostics {
            let tag = match diagnostic.severity {
                DiagnosticSeverity::Error => {
                    errors += 1;
                    "error"
                }
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Message => "message",
            };
            println!(
                "{tag}: {} [{}] {}",
                diagnostic.location, diagnostic.topic, diagnostic.message
            );
        }
        println!(
            "{}: {} control(s), {errors} error(s), {} finding(s)",
            form.display(),
            document.node_count(),
            diagnostics.len()
        );
        Ok(errors == 0)
    }
}

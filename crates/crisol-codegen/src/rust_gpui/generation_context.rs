use crisol_catalog::WidgetCatalog;
use crisol_model::document::FormDocument;

use super::EventBinding;

/// What every writer needs while generating one form.
pub struct GenerationContext<'context> {
    pub document: &'context FormDocument,
    pub catalog: &'context WidgetCatalog,
    pub bindings: &'context [EventBinding],
}

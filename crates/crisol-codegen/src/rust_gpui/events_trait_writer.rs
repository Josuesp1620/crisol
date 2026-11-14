use crisol_model::document::FormDocument;

use super::EventBinding;
use super::event_binding_collector::EventBindingCollector;
use crate::writing::CodeWriter;

/// Writes the trait the user implements: one method per handler.
pub struct EventsTraitWriter<'context> {
    document: &'context FormDocument,
    bindings: &'context [EventBinding],
}

impl<'context> EventsTraitWriter<'context> {
    pub fn new(document: &'context FormDocument, bindings: &'context [EventBinding]) -> Self {
        Self { document, bindings }
    }

    pub fn write(&self, writer: &mut CodeWriter) {
        let form = self.document.form().name.as_str();
        writer.line(&format!(
            "/// The events of `{form}`, implemented in your own file."
        ));
        writer.block(&format!("pub trait {form}Events {{"), "}", |body| {
            for handler in EventBindingCollector::unique_handlers(self.bindings) {
                body.line(&format!("/// {}.", self.sources_of(&handler)));
                body.line(&format!(
                    "fn {handler}(&mut self, window: &mut Window, cx: &mut Context<Self>);"
                ));
            }
        });
    }

    fn sources_of(&self, handler: &str) -> String {
        let mut sources = Vec::new();
        for binding in self.bindings {
            if binding.handler == handler {
                sources.push(format!("`{}`: {}", binding.node_name, binding.event_label));
            }
        }
        sources.join(", ")
    }
}

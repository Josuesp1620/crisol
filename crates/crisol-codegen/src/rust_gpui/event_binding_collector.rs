use crisol_catalog::WidgetCatalog;
use crisol_model::document::{DocumentWalker, FormDocument};

use super::EventBinding;

/// Lists every connected event of a form in reading order.
pub struct EventBindingCollector<'context> {
    document: &'context FormDocument,
    catalog: &'context WidgetCatalog,
}

impl<'context> EventBindingCollector<'context> {
    pub fn new(document: &'context FormDocument, catalog: &'context WidgetCatalog) -> Self {
        Self { document, catalog }
    }

    pub fn collect(&self) -> Vec<EventBinding> {
        let mut bindings = Vec::new();
        for id in DocumentWalker::new(self.document).all() {
            let Ok(node) = self.document.node(id) else {
                continue;
            };
            let schema = self.catalog.schema(node.kind.as_str());
            for (event, handler) in node.events.bindings() {
                let mut event_label = event.clone();
                if let Some(found) = schema.and_then(|widget| widget.event(event)) {
                    found.label.clone_into(&mut event_label);
                }
                bindings.push(EventBinding {
                    node: id,
                    node_name: node.name.to_string(),
                    kind: node.kind.to_string(),
                    event: event.clone(),
                    event_label,
                    handler: handler.to_string(),
                });
            }
        }
        bindings
    }

    /// Handler names without repetition, in the order they first appear.
    pub fn unique_handlers(bindings: &[EventBinding]) -> Vec<String> {
        let mut handlers: Vec<String> = Vec::new();
        for binding in bindings {
            if !handlers.contains(&binding.handler) {
                handlers.push(binding.handler.clone());
            }
        }
        handlers
    }
}

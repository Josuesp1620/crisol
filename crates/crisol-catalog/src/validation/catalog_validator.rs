use crisol_model::document::{DocumentWalker, FormDocument};

use super::accessibility_rule::AccessibilityRule;
use super::containment_rule::ContainmentRule;
use super::event_rule::EventRule;
use super::kind_rule::KindRule;
use super::portability_rule::PortabilityRule;
use super::property_rule::PropertyRule;
use super::{Diagnostic, DiagnosticRule, RuleContext};
use crate::WidgetCatalog;

/// Runs every rule over every node, in reading order.
pub struct CatalogValidator<'catalog> {
    catalog: &'catalog WidgetCatalog,
    rules: Vec<Box<dyn DiagnosticRule>>,
}

impl<'catalog> CatalogValidator<'catalog> {
    pub fn new(catalog: &'catalog WidgetCatalog) -> Self {
        Self {
            catalog,
            rules: vec![
                Box::new(KindRule),
                Box::new(PropertyRule),
                Box::new(EventRule),
                Box::new(ContainmentRule),
                Box::new(PortabilityRule),
                Box::new(AccessibilityRule),
            ],
        }
    }

    pub fn validate(&self, document: &FormDocument) -> Vec<Diagnostic> {
        let mut findings = Vec::new();
        for id in DocumentWalker::new(document).all() {
            let Ok(node) = document.node(id) else {
                continue;
            };
            let context = RuleContext {
                document,
                catalog: self.catalog,
                node,
                schema: self.catalog.schema(node.kind.as_str()),
            };
            for rule in &self.rules {
                rule.check(&context, &mut findings);
            }
        }
        findings.sort_by_key(|finding| finding.severity);
        findings
    }
}

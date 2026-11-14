use crisol_model::document::{FormDocument, Node};

use super::Diagnostic;
use crate::WidgetCatalog;
use crate::schema::WidgetSchema;

/// What a rule sees about the node it checks.
pub struct RuleContext<'context> {
    pub document: &'context FormDocument,
    pub catalog: &'context WidgetCatalog,
    pub node: &'context Node,
    /// The node's schema, when its kind is known.
    pub schema: Option<&'context WidgetSchema>,
}

/// One check applied to every node of a document.
pub trait DiagnosticRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>);
}

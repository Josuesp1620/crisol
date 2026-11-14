use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};

/// Every node must be a control the catalog knows.
pub struct KindRule;

impl DiagnosticRule for KindRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        if context.schema.is_none() {
            findings.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                node: Some(context.node.id),
                location: context.node.name.to_string(),
                message: format!("Unknown control kind {}.", context.node.kind),
                topic: "structure",
            });
        }
    }
}

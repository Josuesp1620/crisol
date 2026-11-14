use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};

/// Warns about what will not work on touch screens and phones.
pub struct PortabilityRule;

impl DiagnosticRule for PortabilityRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        let node = context.node;
        if !node.placement.is_portable() {
            findings.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                node: Some(node.id),
                location: node.name.to_string(),
                message: format!(
                    "{} has an absolute position; phones cannot adapt it.",
                    node.name
                ),
                topic: "portability",
            });
        }
        let has_tooltip = node
            .properties
            .get("tooltip")
            .and_then(|value| value.as_text())
            .is_some_and(|text| !text.is_empty());
        if has_tooltip {
            findings.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                node: Some(node.id),
                location: format!("{}.tooltip", node.name),
                message: format!(
                    "{} has a tooltip. Touch screens have no hover; it shows as help text.",
                    node.name
                ),
                topic: "portability",
            });
        }
    }
}

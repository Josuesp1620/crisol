use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};

/// Inputs without a visible text need an accessible name for screen readers.
pub struct AccessibilityRule;

impl AccessibilityRule {
    const INPUT_KINDS: [&'static str; 2] = ["TextBox", "ComboBox"];
}

impl DiagnosticRule for AccessibilityRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        let node = context.node;
        if !Self::INPUT_KINDS.contains(&node.kind.as_str()) {
            return;
        }
        let named = node
            .properties
            .get("accessible_name")
            .and_then(|value| value.as_text())
            .is_some_and(|text| !text.is_empty());
        if !named {
            findings.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                node: Some(node.id),
                location: node.name.to_string(),
                message: format!(
                    "{} has no accessible name. Screen readers only announce its kind.",
                    node.name
                ),
                topic: "accessibility",
            });
        }
    }
}

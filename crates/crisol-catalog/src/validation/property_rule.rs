use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};

/// Every stored property must exist on the control and hold a value of its type.
pub struct PropertyRule;

impl DiagnosticRule for PropertyRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        let Some(schema) = context.schema else {
            return;
        };
        for (name, value) in context.node.properties.entries() {
            let message = match schema.property(name) {
                None => format!("{} has no property {name}.", schema.label),
                Some(property) if !property.value_type.accepts(value) => {
                    format!("The value of {} is not valid.", property.label)
                }
                Some(_) => continue,
            };
            findings.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                node: Some(context.node.id),
                location: format!("{}.{name}", context.node.name),
                message,
                topic: "properties",
            });
        }
    }
}

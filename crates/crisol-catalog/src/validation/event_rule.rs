use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};

/// Every bound event must exist on the control.
pub struct EventRule;

impl DiagnosticRule for EventRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        let Some(schema) = context.schema else {
            return;
        };
        for (event, _handler) in context.node.events.bindings() {
            if schema.event(event).is_none() {
                findings.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    node: Some(context.node.id),
                    location: format!("{}.{event}", context.node.name),
                    message: format!("{} has no event {event}.", schema.label),
                    topic: "events",
                });
            }
        }
    }
}

use crisol_model::layout::Placement;

use super::{Diagnostic, DiagnosticRule, DiagnosticSeverity, RuleContext};
use crate::schema::ContainerRule;

/// Children only go into containers, placed the way their container places them.
pub struct ContainmentRule;

impl ContainmentRule {
    fn placement_fits(rule: ContainerRule, placement: &Placement) -> bool {
        matches!(
            (rule, placement),
            (ContainerRule::Flow, Placement::Flow { .. })
                | (ContainerRule::Grid, Placement::Cell { .. })
                | (ContainerRule::Absolute, Placement::Absolute { .. })
        )
    }
}

impl DiagnosticRule for ContainmentRule {
    fn check(&self, context: &RuleContext<'_>, findings: &mut Vec<Diagnostic>) {
        let Some(parent_id) = context.node.parent() else {
            return;
        };
        let Ok(parent) = context.document.node(parent_id) else {
            return;
        };
        let Some(parent_schema) = context.catalog.schema(parent.kind.as_str()) else {
            return;
        };
        let message = if parent_schema.container.accepts_children() {
            if Self::placement_fits(parent_schema.container, &context.node.placement) {
                return;
            }
            format!(
                "{} is placed in a way {} does not support.",
                context.node.name, parent.name
            )
        } else {
            format!("{} cannot hold other controls.", parent.name)
        };
        findings.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            node: Some(context.node.id),
            location: context.node.name.to_string(),
            message,
            topic: "structure",
        });
    }
}

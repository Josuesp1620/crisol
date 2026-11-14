use crisol_model::identity::NodeId;

use super::DiagnosticSeverity;

/// One finding of the validator, shown in the designer's error list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub node: Option<NodeId>,
    /// Where it happened, as the user reads it (`btn_save`).
    pub location: String,
    pub message: String,
    /// Short topic tag (`portability`, `accessibility`).
    pub topic: &'static str,
}

impl Diagnostic {
    pub fn is_error(&self) -> bool {
        self.severity == DiagnosticSeverity::Error
    }
}

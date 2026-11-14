/// How serious a diagnostic is. Errors block code generation; warnings and messages do not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Message,
}

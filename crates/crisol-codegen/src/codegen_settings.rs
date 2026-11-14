/// The single place where code generation values live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenSettings {
    /// Suffix of generated files: `customer_form` becomes `customer_form.generated.rs`.
    pub generated_suffix: &'static str,
    /// Spaces per indentation level.
    pub indent_width: usize,
    /// Name of the component library the generated code uses.
    pub runtime_crate: &'static str,
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            generated_suffix: ".generated.rs",
            indent_width: 4,
            runtime_crate: "crisol_ui",
        }
    }
}

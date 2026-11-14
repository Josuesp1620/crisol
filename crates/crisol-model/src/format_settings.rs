/// The single place where the `.crisol` file format values live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatSettings {
    /// Format written by this version and the only one it reads.
    pub format_version: &'static str,
    /// Extension of form files, without the dot. Always `crisol`, never `json`.
    pub file_extension: &'static str,
    /// Spaces per indentation level in the canonical file.
    pub indent_width: usize,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            format_version: "crisol/1",
            file_extension: "crisol",
            indent_width: 2,
        }
    }
}

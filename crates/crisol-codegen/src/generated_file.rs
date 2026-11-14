/// A file produced by a generator, ready to be written next to the user's code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub file_name: String,
    pub content: String,
}

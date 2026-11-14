use crisol_model::document::FormDocument;

use crate::{CodegenResult, GeneratedFile};

/// Turns a form into source code for one platform.
pub trait CodeGenerator {
    fn generate(&self, document: &FormDocument) -> CodegenResult<GeneratedFile>;
}

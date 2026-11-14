use crisol_model::document::FormDocument;

use crate::writing::NameConverter;
use crate::{CodegenError, CodegenResult, CodegenSettings};

/// Creates the user's handlers file and adds the methods new events need. It only ever
/// adds code: existing methods are never changed or removed.
pub struct HandlerStubWriter<'document> {
    document: &'document FormDocument,
    settings: CodegenSettings,
    module: String,
}

impl<'document> HandlerStubWriter<'document> {
    /// Writes for the module named after the form type (`CustomerForm` gives `customer_form`).
    pub fn new(document: &'document FormDocument, settings: CodegenSettings) -> Self {
        let module = NameConverter::snake_case(document.form().name.as_str());
        Self {
            document,
            settings,
            module,
        }
    }

    /// Writes for the module named after the form file (`customer.crisol` gives `customer`).
    pub fn with_module(mut self, module: &str) -> Self {
        module.clone_into(&mut self.module);
        self
    }

    /// `customer_events.rs`.
    pub fn file_name(&self) -> String {
        format!("{}_events.rs", self.module)
    }

    /// A new handlers file with an empty method for every handler.
    pub fn create(&self, handlers: &[String]) -> String {
        let form = self.document.form().name.as_str();
        let module = &self.module;
        let mut text = String::new();
        text.push_str("use gpui::{Context, Window};\n\n");
        text.push_str(&format!(
            "use super::{module}::{{{form}, {form}Events}};\n\n"
        ));
        text.push_str(&format!("impl {form}Events for {form} {{\n"));
        for (position, handler) in handlers.iter().enumerate() {
            if position > 0 {
                text.push('\n');
            }
            text.push_str(&self.stub(handler));
        }
        text.push_str("}\n");
        text
    }

    /// The existing file with an empty method appended for each handler it lacks.
    pub fn extend(&self, existing: &str, handlers: &[String]) -> CodegenResult<String> {
        let form = self.document.form().name.as_str();
        let header = format!("impl {form}Events for {form}");
        let start = existing
            .find(&header)
            .ok_or_else(|| CodegenError::MissingHandlersImpl(format!("{form}Events")))?;
        let closing = Self::closing_brace(existing, start)
            .ok_or_else(|| CodegenError::MissingHandlersImpl(format!("{form}Events")))?;
        let mut additions = String::new();
        for handler in handlers {
            if !existing[start..closing].contains(&format!("fn {handler}(")) {
                additions.push('\n');
                additions.push_str(&self.stub(handler));
            }
        }
        let mut result = String::with_capacity(existing.len() + additions.len());
        result.push_str(existing[..closing].trim_end_matches([' ', '\t']));
        result.push_str(&additions);
        result.push_str(&existing[closing..]);
        Ok(result)
    }

    fn stub(&self, handler: &str) -> String {
        let indent = " ".repeat(self.settings.indent_width);
        format!(
            "{indent}fn {handler}(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {{}}\n"
        )
    }

    /// Byte position of the `}` that closes the block opened after `start`.
    fn closing_brace(text: &str, start: usize) -> Option<usize> {
        let mut depth = 0;
        for (offset, character) in text[start..].char_indices() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(start + offset);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

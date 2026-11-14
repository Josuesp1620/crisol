use crisol_model::document::{FormDocument, FormInfo};

use crate::{Command, CommandResult};

/// Changes the form itself: type name, title, design size or state type.
#[derive(Debug, Clone)]
pub struct SetFormInfoCommand {
    pub form: FormInfo,
}

impl Command for SetFormInfoCommand {
    fn id(&self) -> &'static str {
        "form.set"
    }

    fn label(&self) -> String {
        "Change form".to_owned()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let previous = std::mem::replace(document.form_mut(), self.form.clone());
        Ok(Box::new(SetFormInfoCommand { form: previous }))
    }
}

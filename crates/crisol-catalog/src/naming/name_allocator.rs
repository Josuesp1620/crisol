use crisol_model::document::FormDocument;
use crisol_model::identity::Identifier;

/// Gives new controls the first free name made of their prefix and a number (`btn_1`).
pub struct NameAllocator<'document> {
    document: &'document FormDocument,
}

impl<'document> NameAllocator<'document> {
    pub fn new(document: &'document FormDocument) -> Self {
        Self { document }
    }

    pub fn next_name(&self, prefix: &str) -> Identifier {
        let mut number = 1;
        loop {
            let candidate = format!("{prefix}_{number}");
            if self.document.find_by_name(&candidate).is_none()
                && let Ok(identifier) = Identifier::parse(&candidate)
            {
                return identifier;
            }
            number += 1;
        }
    }
}

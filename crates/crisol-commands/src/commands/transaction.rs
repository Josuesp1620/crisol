use crisol_model::document::FormDocument;

use crate::{Command, CommandResult};

/// Several commands applied as one undo step. If one fails, the ones already applied are
/// reverted, so the document never stays half changed.
#[derive(Debug)]
pub struct Transaction {
    pub label: String,
    pub commands: Vec<Box<dyn Command>>,
}

impl Transaction {
    pub fn new(label: &str, commands: Vec<Box<dyn Command>>) -> Self {
        Self {
            label: label.to_owned(),
            commands,
        }
    }

    fn revert(document: &mut FormDocument, inverses: &[Box<dyn Command>]) {
        for inverse in inverses.iter().rev() {
            if inverse.apply(document).is_err() {
                break;
            }
        }
    }
}

impl Command for Transaction {
    fn id(&self) -> &'static str {
        "transaction"
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let mut inverses: Vec<Box<dyn Command>> = Vec::with_capacity(self.commands.len());
        for command in &self.commands {
            match command.apply(document) {
                Ok(inverse) => inverses.push(inverse),
                Err(error) => {
                    Self::revert(document, &inverses);
                    return Err(error);
                }
            }
        }
        inverses.reverse();
        Ok(Box::new(Transaction::new(&self.label, inverses)))
    }
}

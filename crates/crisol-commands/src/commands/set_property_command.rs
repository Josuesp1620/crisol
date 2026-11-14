use crisol_model::document::FormDocument;
use crisol_model::identity::NodeId;
use crisol_model::properties::PropertyValue;

use crate::{Command, CommandResult};

/// Sets a property, or resets it to the control's default when `value` is `None`.
#[derive(Debug, Clone)]
pub struct SetPropertyCommand {
    pub node: NodeId,
    pub property: String,
    pub value: Option<PropertyValue>,
}

impl Command for SetPropertyCommand {
    fn id(&self) -> &'static str {
        "property.set"
    }

    fn label(&self) -> String {
        format!("Change {}", self.property)
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let properties = &mut document.node_mut(self.node)?.properties;
        let previous = match &self.value {
            Some(value) => properties.set(&self.property, value.clone()),
            None => properties.reset(&self.property),
        };
        Ok(Box::new(SetPropertyCommand {
            node: self.node,
            property: self.property.clone(),
            value: previous,
        }))
    }
}

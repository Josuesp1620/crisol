use crisol_model::document::FormDocument;
use crisol_model::identity::{Identifier, NodeId};

use crate::{Command, CommandResult};

/// Connects an event to a handler method, or disconnects it when `handler` is `None`.
#[derive(Debug, Clone)]
pub struct BindEventCommand {
    pub node: NodeId,
    pub event: String,
    pub handler: Option<Identifier>,
}

impl Command for BindEventCommand {
    fn id(&self) -> &'static str {
        "event.bind"
    }

    fn label(&self) -> String {
        match &self.handler {
            Some(handler) => format!("Connect {} to {handler}", self.event),
            None => format!("Disconnect {}", self.event),
        }
    }

    fn apply(&self, document: &mut FormDocument) -> CommandResult<Box<dyn Command>> {
        let events = &mut document.node_mut(self.node)?.events;
        let previous = match &self.handler {
            Some(handler) => events.bind(&self.event, handler.clone()),
            None => events.unbind(&self.event),
        };
        Ok(Box::new(BindEventCommand {
            node: self.node,
            event: self.event.clone(),
            handler: previous,
        }))
    }
}

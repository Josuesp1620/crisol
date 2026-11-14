use crate::events::EventMap;
use crate::identity::{Identifier, NodeId};
use crate::layout::Placement;
use crate::properties::PropertyMap;

use super::WidgetKind;

/// One control of the form. Its children and parent are ids into the same document.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: WidgetKind,
    pub name: Identifier,
    pub note: Option<String>,
    pub properties: PropertyMap,
    pub placement: Placement,
    pub events: EventMap,
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: Vec<NodeId>,
}

impl Node {
    pub fn new(kind: WidgetKind, name: Identifier, placement: Placement) -> Self {
        Self {
            id: NodeId::generate(),
            kind,
            name,
            note: None,
            properties: PropertyMap::default(),
            placement,
            events: EventMap::default(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
}

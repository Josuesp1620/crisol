use serde::{Deserialize, Serialize};

use crate::document::WidgetKind;
use crate::events::EventMap;
use crate::identity::{Identifier, NodeId};
use crate::layout::Placement;
use crate::properties::PropertyMap;

/// A node as written in the file: its children nested inside it, empty parts left out.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct NodeRecord {
    pub id: NodeId,
    pub kind: WidgetKind,
    pub name: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "PropertyMap::is_empty")]
    pub properties: PropertyMap,
    #[serde(default)]
    pub placement: Placement,
    #[serde(default, skip_serializing_if = "EventMap::is_empty")]
    pub events: EventMap,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeRecord>,
}

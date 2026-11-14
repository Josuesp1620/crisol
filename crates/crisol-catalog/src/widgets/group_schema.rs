use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema};

/// A titled frame around related controls, stacked vertically.
pub struct GroupSchema;

impl GroupSchema {
    pub fn build() -> WidgetSchema {
        let mut properties = vec![PropertySchema::text(
            "title",
            "Title",
            "The title shown on the frame.",
            PropertyCategory::Appearance,
            "Group",
        )];
        properties.extend(CommonProperties::for_container(8, 12));
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "Group",
            label: "Group",
            description: "A titled frame around related controls.",
            icon: "group",
            toolbox_group: ToolboxGroup::Containers,
            name_prefix: "grp",
            container: ContainerRule::Flow,
            default_size: Size::new(280, 160),
            desktop_only: false,
            properties,
            events: Vec::new(),
            default_event: None,
        }
    }
}

use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema};

/// Places its children one after another; works on every platform.
pub struct StackSchema;

impl StackSchema {
    pub const DIRECTIONS: [&'static str; 2] = ["vertical", "horizontal"];
    pub const ALIGNMENTS: [&'static str; 4] = ["start", "center", "end", "stretch"];

    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::choice(
                "direction",
                "Direction",
                "Whether children go top to bottom or left to right.",
                PropertyCategory::Layout,
                &Self::DIRECTIONS,
            ),
            PropertySchema::choice(
                "alignment",
                "Alignment",
                "Where children sit across the direction of the stack.",
                PropertyCategory::Layout,
                &Self::ALIGNMENTS,
            ),
        ];
        properties.extend(CommonProperties::for_container(8, 0));
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "Stack",
            label: "Stack",
            description: "Places its children one after another.",
            icon: "stack",
            toolbox_group: ToolboxGroup::Containers,
            name_prefix: "stk",
            container: ContainerRule::Flow,
            default_size: Size::new(240, 120),
            desktop_only: false,
            properties,
            events: Vec::new(),
            default_event: None,
        }
    }
}

use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema};

/// Text that names or explains other controls.
pub struct LabelSchema;

impl LabelSchema {
    pub fn build() -> WidgetSchema {
        let mut properties = vec![PropertySchema::text(
            "text",
            "Text",
            "The text the label shows.",
            PropertyCategory::Appearance,
            "Label",
        )];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "Label",
            label: "Label",
            description: "Text that names or explains other controls.",
            icon: "label",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "lbl",
            container: ContainerRule::Leaf,
            default_size: Size::new(100, 20),
            desktop_only: false,
            properties,
            events: Vec::new(),
            default_event: None,
        }
    }
}

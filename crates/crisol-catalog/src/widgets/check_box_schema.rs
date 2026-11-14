use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{
    ContainerRule, EventSchema, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema,
};

/// A yes or no choice. iOS shows it as a switch.
pub struct CheckBoxSchema;

impl CheckBoxSchema {
    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::text(
                "text",
                "Text",
                "The text next to the box.",
                PropertyCategory::Appearance,
                "Check box",
            ),
            PropertySchema::boolean(
                "checked",
                "Checked",
                "Whether the box starts checked.",
                PropertyCategory::Data,
                false,
            ),
        ];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "CheckBox",
            label: "Check box",
            description: "A yes or no choice.",
            icon: "checkbox",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "chk",
            container: ContainerRule::Leaf,
            default_size: Size::new(140, 20),
            desktop_only: false,
            properties,
            events: vec![EventSchema::new(
                "changed",
                "Changed",
                "Raised when the user checks or clears the box.",
            )],
            default_event: Some("changed"),
        }
    }
}

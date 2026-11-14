use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{
    ContainerRule, EventSchema, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema,
};

/// One option of a set where only one can be chosen.
pub struct RadioButtonSchema;

impl RadioButtonSchema {
    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::text(
                "text",
                "Text",
                "The text next to the option.",
                PropertyCategory::Appearance,
                "Option",
            ),
            PropertySchema::text(
                "group",
                "Group",
                "Options with the same group exclude each other.",
                PropertyCategory::Behavior,
                "options",
            ),
            PropertySchema::boolean(
                "checked",
                "Checked",
                "Whether this option starts chosen.",
                PropertyCategory::Data,
                false,
            ),
        ];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "RadioButton",
            label: "Option",
            description: "One option of a set where only one can be chosen.",
            icon: "radio",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "opt",
            container: ContainerRule::Leaf,
            default_size: Size::new(140, 20),
            desktop_only: false,
            properties,
            events: vec![EventSchema::new(
                "checked",
                "Checked",
                "Raised when the user chooses this option.",
            )],
            default_event: Some("checked"),
        }
    }
}

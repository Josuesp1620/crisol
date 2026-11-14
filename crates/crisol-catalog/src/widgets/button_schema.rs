use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{
    ContainerRule, EventSchema, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema,
};

/// A button that runs an action when clicked.
pub struct ButtonSchema;

impl ButtonSchema {
    pub const VARIANTS: [&'static str; 3] = ["secondary", "primary", "ghost"];

    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::text(
                "text",
                "Text",
                "The text the button shows.",
                PropertyCategory::Appearance,
                "Button",
            ),
            PropertySchema::choice(
                "variant",
                "Variant",
                "Visual weight: primary for the main action of the form.",
                PropertyCategory::Appearance,
                &Self::VARIANTS,
            ),
            PropertySchema::boolean(
                "is_default",
                "Default (Enter)",
                "Pressing Enter anywhere in the form clicks this button.",
                PropertyCategory::Behavior,
                false,
            ),
            PropertySchema::boolean(
                "is_cancel",
                "Cancel (Esc)",
                "Pressing Esc anywhere in the form clicks this button.",
                PropertyCategory::Behavior,
                false,
            ),
        ];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "Button",
            label: "Button",
            description: "Runs an action when clicked.",
            icon: "button",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "btn",
            container: ContainerRule::Leaf,
            default_size: Size::new(100, 32),
            desktop_only: false,
            properties,
            events: vec![EventSchema::new(
                "click",
                "Click",
                "Raised when the user clicks the button.",
            )],
            default_event: Some("click"),
        }
    }
}

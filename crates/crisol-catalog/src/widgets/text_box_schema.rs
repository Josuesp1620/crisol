use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{
    ContainerRule, EventSchema, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema,
};

/// A field where the user types text.
pub struct TextBoxSchema;

impl TextBoxSchema {
    pub const INPUT_KINDS: [&'static str; 5] = ["text", "email", "number", "password", "multiline"];

    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::text(
                "text",
                "Text",
                "Initial text of the field.",
                PropertyCategory::Appearance,
                "",
            ),
            PropertySchema::text(
                "placeholder",
                "Placeholder",
                "Hint shown while the field is empty.",
                PropertyCategory::Appearance,
                "",
            ),
            PropertySchema::choice(
                "input_kind",
                "Input kind",
                "What the field holds; phones show the matching keyboard.",
                PropertyCategory::Behavior,
                &Self::INPUT_KINDS,
            ),
            PropertySchema::integer(
                "max_length",
                "Maximum length",
                "Most characters the field accepts; 0 means no limit.",
                PropertyCategory::Behavior,
                (0, 1_000_000),
                0,
            ),
            PropertySchema::boolean(
                "required",
                "Required",
                "The form cannot be saved while the field is empty.",
                PropertyCategory::Behavior,
                false,
            ),
            PropertySchema::boolean(
                "read_only",
                "Read only",
                "The user can select the text but not change it.",
                PropertyCategory::Behavior,
                false,
            ),
        ];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "TextBox",
            label: "Text box",
            description: "A field where the user types text.",
            icon: "textbox",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "txt",
            container: ContainerRule::Leaf,
            default_size: Size::new(200, 30),
            desktop_only: false,
            properties,
            events: vec![
                EventSchema::new("changed", "Changed", "Raised after every edit of the text."),
                EventSchema::new(
                    "submitted",
                    "Enter pressed",
                    "Raised when the user presses Enter.",
                ),
                CommonProperties::focus_gained_event(),
                CommonProperties::focus_lost_event(),
            ],
            default_event: Some("changed"),
        }
    }
}

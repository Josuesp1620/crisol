use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{
    ContainerRule, EventSchema, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema,
};

/// A drop-down list to pick one option.
pub struct ComboBoxSchema;

impl ComboBoxSchema {
    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::text_list(
                "items",
                "Items",
                "The options the list shows. Use Edit items… to add, remove or reorder them.",
                PropertyCategory::Appearance,
            ),
            PropertySchema::text(
                "placeholder",
                "Placeholder",
                "Hint shown while nothing is selected.",
                PropertyCategory::Appearance,
                "",
            ),
            PropertySchema::boolean(
                "required",
                "Required",
                "The form cannot be saved while nothing is selected.",
                PropertyCategory::Behavior,
                false,
            ),
            PropertySchema::boolean(
                "allow_typing",
                "Allow typing",
                "The user may type a value that is not in the list.",
                PropertyCategory::Behavior,
                false,
            ),
            PropertySchema::text(
                "selected",
                "Initial value",
                "The option selected when the form opens.",
                PropertyCategory::Data,
                "",
            ),
        ];
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "ComboBox",
            label: "Combo box",
            description: "A drop-down list to pick one option.",
            icon: "combo",
            toolbox_group: ToolboxGroup::CommonControls,
            name_prefix: "cmb",
            container: ContainerRule::Leaf,
            default_size: Size::new(180, 30),
            desktop_only: false,
            properties,
            events: vec![
                EventSchema::new(
                    "selection_changed",
                    "Selection changed",
                    "Raised when the user picks another option.",
                ),
                EventSchema::new("opened", "List opened", "Raised when the list opens."),
                CommonProperties::focus_gained_event(),
                CommonProperties::focus_lost_event(),
            ],
            default_event: Some("selection_changed"),
        }
    }
}

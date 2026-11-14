use crate::schema::{EventSchema, PropertyCategory, PropertySchema};

/// Properties and events every visible control shares.
pub struct CommonProperties;

impl CommonProperties {
    pub fn for_control() -> Vec<PropertySchema> {
        vec![
            PropertySchema::boolean(
                "visible",
                "Visible",
                "Whether the control is shown.",
                PropertyCategory::Behavior,
                true,
            ),
            PropertySchema::boolean(
                "enabled",
                "Enabled",
                "Whether the user can interact with the control.",
                PropertyCategory::Behavior,
                true,
            ),
            PropertySchema::text(
                "tooltip",
                "Tooltip",
                "Help text shown when the pointer rests on the control. Touch screens show it as help text.",
                PropertyCategory::Behavior,
                "",
            ),
            PropertySchema::text(
                "accessible_name",
                "Accessible name",
                "What screen readers announce for this control.",
                PropertyCategory::Accessibility,
                "",
            ),
        ]
    }

    pub fn focus_lost_event() -> EventSchema {
        EventSchema::new(
            "focus_lost",
            "Focus lost",
            "Raised when the control stops receiving the keyboard.",
        )
    }

    pub fn focus_gained_event() -> EventSchema {
        EventSchema::new(
            "focus_gained",
            "Focus gained",
            "Raised when the control starts receiving the keyboard.",
        )
    }

    /// Container layout: space between children and around them.
    pub fn for_container(default_spacing: i64, default_padding: i64) -> Vec<PropertySchema> {
        vec![
            PropertySchema::integer(
                "spacing",
                "Spacing",
                "Space between children, in logical pixels.",
                PropertyCategory::Layout,
                (0, 200),
                default_spacing,
            ),
            PropertySchema::integer(
                "padding",
                "Padding",
                "Space between the edge and the children, in logical pixels.",
                PropertyCategory::Layout,
                (0, 200),
                default_padding,
            ),
        ]
    }
}

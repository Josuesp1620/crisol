use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, ToolboxGroup, WidgetSchema};

/// The root of every form: a vertical stack filling the window.
pub struct FormSchema;

impl FormSchema {
    pub const KIND: &'static str = "Form";

    pub fn build() -> WidgetSchema {
        WidgetSchema {
            kind: Self::KIND,
            label: "Form",
            description: "The window of the form.",
            icon: "form",
            toolbox_group: ToolboxGroup::Hidden,
            name_prefix: "frm",
            container: ContainerRule::Flow,
            default_size: Size::new(520, 380),
            desktop_only: false,
            properties: CommonProperties::for_container(16, 24),
            events: Vec::new(),
            default_event: None,
        }
    }
}

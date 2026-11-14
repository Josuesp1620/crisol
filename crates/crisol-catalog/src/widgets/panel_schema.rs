use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, ToolboxGroup, WidgetSchema};

/// Places its children at fixed positions with anchors, like Windows Forms. Desktop only.
pub struct PanelSchema;

impl PanelSchema {
    pub fn build() -> WidgetSchema {
        WidgetSchema {
            kind: "Panel",
            label: "Absolute panel",
            description: "Places its children at fixed positions with anchors. Desktop only.",
            icon: "panel",
            toolbox_group: ToolboxGroup::Containers,
            name_prefix: "pnl",
            container: ContainerRule::Absolute,
            default_size: Size::new(320, 200),
            desktop_only: true,
            properties: CommonProperties::for_control(),
            events: Vec::new(),
            default_event: None,
        }
    }
}

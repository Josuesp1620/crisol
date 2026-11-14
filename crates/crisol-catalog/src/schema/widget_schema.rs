use crisol_model::geometry::Size;

use super::{ContainerRule, EventSchema, PropertySchema, ToolboxGroup};

/// Everything Crisol knows about one kind of control.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetSchema {
    /// Catalog name written in `.crisol` files (`Button`).
    pub kind: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    /// Icon name shared with the designer's icon set.
    pub icon: &'static str,
    pub toolbox_group: ToolboxGroup,
    /// Prefix of new control names (`btn` gives `btn_1`).
    pub name_prefix: &'static str,
    pub container: ContainerRule,
    /// Size of a new control dropped on an absolute panel.
    pub default_size: Size,
    /// Only Windows, macOS and Linux can show it.
    pub desktop_only: bool,
    pub properties: Vec<PropertySchema>,
    pub events: Vec<EventSchema>,
    /// Event bound by double-clicking the control in the designer.
    pub default_event: Option<&'static str>,
}

impl WidgetSchema {
    pub fn property(&self, name: &str) -> Option<&PropertySchema> {
        self.properties
            .iter()
            .find(|property| property.name == name)
    }

    pub fn event(&self, name: &str) -> Option<&EventSchema> {
        self.events.iter().find(|event| event.name == name)
    }
}

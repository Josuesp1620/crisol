use crate::schema::{ToolboxGroup, WidgetSchema};
use crate::widgets::{
    ButtonSchema, CheckBoxSchema, ComboBoxSchema, FormSchema, GridSchema, GroupSchema, LabelSchema,
    PanelSchema, RadioButtonSchema, StackSchema, TextBoxSchema,
};

/// The set of controls a document may use, in toolbox order.
pub struct WidgetCatalog {
    widgets: Vec<WidgetSchema>,
}

impl WidgetCatalog {
    pub fn standard() -> Self {
        Self {
            widgets: vec![
                LabelSchema::build(),
                ButtonSchema::build(),
                TextBoxSchema::build(),
                CheckBoxSchema::build(),
                RadioButtonSchema::build(),
                ComboBoxSchema::build(),
                StackSchema::build(),
                GridSchema::build(),
                GroupSchema::build(),
                PanelSchema::build(),
                FormSchema::build(),
            ],
        }
    }

    pub fn schema(&self, kind: &str) -> Option<&WidgetSchema> {
        self.widgets.iter().find(|widget| widget.kind == kind)
    }

    pub fn widgets(&self) -> &[WidgetSchema] {
        &self.widgets
    }

    /// The controls of one toolbox group, in toolbox order.
    pub fn in_group(&self, group: ToolboxGroup) -> Vec<&WidgetSchema> {
        let mut members = Vec::new();
        for widget in &self.widgets {
            if widget.toolbox_group == group {
                members.push(widget);
            }
        }
        members
    }
}

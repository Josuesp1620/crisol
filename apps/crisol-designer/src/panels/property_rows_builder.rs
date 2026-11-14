use crisol_catalog::WidgetCatalog;
use crisol_catalog::schema::PropertyCategory;
use crisol_model::document::Node;
use crisol_model::identity::NodeId;
use crisol_model::layout::{Placement, SizeMode};

use super::PropertyRow;
use crate::designer_texts::DesignerTexts;
use crate::workspace::{DesignerWorkspace, PropertyKey, PropertyTab};

/// Lists the rows the property grid shows for the selected control.
pub struct PropertyRowsBuilder<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> PropertyRowsBuilder<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn rows(&self, node: NodeId) -> Vec<PropertyRow> {
        let Ok(found) = self.workspace.session().document().node(node) else {
            return Vec::new();
        };
        match self.workspace.property_tab() {
            PropertyTab::Properties => self.property_rows(node, found),
            PropertyTab::Events => self.event_rows(found),
        }
    }

    fn property_rows(&self, node: NodeId, found: &Node) -> Vec<PropertyRow> {
        let catalog: &WidgetCatalog = self.workspace.catalog();
        let mut rows = Vec::new();
        let is_root = node == self.workspace.session().document().root_id();
        if let Some(schema) = catalog.schema(found.kind.as_str()) {
            for category in PropertyCategory::ORDER {
                for property in &schema.properties {
                    if property.category != category {
                        continue;
                    }
                    let key = PropertyKey::Property(property.name.to_owned());
                    rows.push(PropertyRow {
                        value: self.workspace.current_text(node, &key),
                        key,
                        category: category.label().to_owned(),
                        label: property.label.to_owned(),
                        description: property.description.to_owned(),
                        modified: found.properties.get(property.name).is_some(),
                        is_event: false,
                        read_only: false,
                    });
                }
            }
        }
        rows.extend(self.design_rows(node, found, is_root));
        rows
    }

    fn design_rows(&self, node: NodeId, found: &Node, is_root: bool) -> Vec<PropertyRow> {
        let category = DesignerTexts::DESIGN_CATEGORY.to_owned();
        let mut rows = vec![PropertyRow {
            key: PropertyKey::Name,
            category: category.clone(),
            label: DesignerTexts::NAME_PROPERTY.to_owned(),
            value: found.name.to_string(),
            description: DesignerTexts::NAME_DESCRIPTION.to_owned(),
            modified: true,
            is_event: false,
            read_only: false,
        }];
        if is_root {
            for (key, label, description) in [
                (
                    PropertyKey::FormTitle,
                    DesignerTexts::FORM_TITLE,
                    DesignerTexts::FORM_TITLE_DESCRIPTION,
                ),
                (
                    PropertyKey::FormType,
                    DesignerTexts::FORM_TYPE,
                    DesignerTexts::FORM_TYPE_DESCRIPTION,
                ),
                (
                    PropertyKey::FormWidth,
                    DesignerTexts::FORM_WIDTH,
                    DesignerTexts::FORM_SIZE_DESCRIPTION,
                ),
                (
                    PropertyKey::FormHeight,
                    DesignerTexts::FORM_HEIGHT,
                    DesignerTexts::FORM_SIZE_DESCRIPTION,
                ),
            ] {
                rows.push(PropertyRow {
                    value: self.workspace.current_text(node, &key),
                    key,
                    category: category.clone(),
                    label: label.to_owned(),
                    description: description.to_owned(),
                    modified: true,
                    is_event: false,
                    read_only: false,
                });
            }
        } else {
            rows.push(PropertyRow {
                key: PropertyKey::Property(String::new()),
                category,
                label: DesignerTexts::PLACEMENT_PROPERTY.to_owned(),
                value: Self::placement_text(&found.placement),
                description: DesignerTexts::PLACEMENT_DESCRIPTION.to_owned(),
                modified: false,
                is_event: false,
                read_only: true,
            });
        }
        rows
    }

    fn event_rows(&self, found: &Node) -> Vec<PropertyRow> {
        let mut rows = Vec::new();
        let Some(schema) = self.workspace.catalog().schema(found.kind.as_str()) else {
            return rows;
        };
        for event in &schema.events {
            let handler = found
                .events
                .handler_for(event.name)
                .map(ToString::to_string);
            rows.push(PropertyRow {
                key: PropertyKey::Event(event.name.to_owned()),
                category: schema.label.to_owned(),
                label: event.label.to_owned(),
                modified: handler.is_some(),
                value: handler.unwrap_or_else(|| "—".to_owned()),
                description: format!("{} {}", event.description, DesignerTexts::EVENTS_HINT),
                is_event: true,
                read_only: false,
            });
        }
        rows
    }

    fn placement_text(placement: &Placement) -> String {
        match placement {
            Placement::Flow { width, height } => format!(
                "In order · {} × {}",
                Self::mode(*width),
                Self::mode(*height)
            ),
            Placement::Cell { cell, width } => format!(
                "Row {}, column {} · {}",
                cell.row + 1,
                cell.column + 1,
                Self::mode(*width)
            ),
            Placement::Absolute { rect, .. } => {
                format!(
                    "{}, {} · {} × {}",
                    rect.left(),
                    rect.top(),
                    rect.size.width,
                    rect.size.height
                )
            }
        }
    }

    fn mode(mode: SizeMode) -> String {
        match mode {
            SizeMode::Fixed(pixels) => pixels.to_string(),
            SizeMode::Hug => "hug".to_owned(),
            SizeMode::Fill => "fill".to_owned(),
        }
    }
}

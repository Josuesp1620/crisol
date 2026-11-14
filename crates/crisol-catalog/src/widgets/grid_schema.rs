use crisol_model::geometry::Size;

use super::CommonProperties;
use crate::schema::{ContainerRule, PropertyCategory, PropertySchema, ToolboxGroup, WidgetSchema};

/// Places its children in rows and columns; works on every platform.
pub struct GridSchema;

impl GridSchema {
    pub const DEFAULT_COLUMNS: i64 = 2;

    pub fn build() -> WidgetSchema {
        let mut properties = vec![
            PropertySchema::integer(
                "columns",
                "Columns",
                "How many columns the grid has.",
                PropertyCategory::Layout,
                (1, 12),
                Self::DEFAULT_COLUMNS,
            ),
            PropertySchema::integer(
                "first_column_width",
                "First column width",
                "Width of the first column in logical pixels, usually the labels; 0 shares the space evenly.",
                PropertyCategory::Layout,
                (0, 1000),
                0,
            ),
            PropertySchema::integer(
                "row_spacing",
                "Row spacing",
                "Space between rows, in logical pixels.",
                PropertyCategory::Layout,
                (0, 200),
                10,
            ),
        ];
        properties.extend(CommonProperties::for_container(16, 0));
        properties.extend(CommonProperties::for_control());
        WidgetSchema {
            kind: "Grid",
            label: "Grid",
            description: "Places its children in rows and columns.",
            icon: "grid",
            toolbox_group: ToolboxGroup::Containers,
            name_prefix: "grd",
            container: ContainerRule::Grid,
            default_size: Size::new(320, 160),
            desktop_only: false,
            properties,
            events: Vec::new(),
            default_event: None,
        }
    }
}

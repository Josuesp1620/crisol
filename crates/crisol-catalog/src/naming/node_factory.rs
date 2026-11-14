use crisol_model::document::{FormDocument, Node, WidgetKind};
use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;
use crisol_model::layout::{AnchorSet, GridCell, Placement, SizeMode};

use super::NameAllocator;
use crate::WidgetCatalog;
use crate::schema::{ContainerRule, WidgetSchema};
use crate::widgets::GridSchema;

/// Builds a new node of a given kind, named and placed for the container it goes into.
pub struct NodeFactory<'context> {
    catalog: &'context WidgetCatalog,
    document: &'context FormDocument,
}

impl<'context> NodeFactory<'context> {
    pub fn new(catalog: &'context WidgetCatalog, document: &'context FormDocument) -> Self {
        Self { catalog, document }
    }

    /// A node for `schema` placed in `parent`; `drop_point` is used by absolute panels.
    pub fn create(&self, schema: &WidgetSchema, parent: NodeId, drop_point: Point) -> Node {
        let name = NameAllocator::new(self.document).next_name(schema.name_prefix);
        let placement = self.placement_for(schema, parent, drop_point);
        Node::new(WidgetKind::new(schema.kind), name, placement)
    }

    fn placement_for(&self, schema: &WidgetSchema, parent: NodeId, drop_point: Point) -> Placement {
        match self.container_rule_of(parent) {
            ContainerRule::Absolute => Placement::Absolute {
                rect: Rect::new(
                    drop_point.x,
                    drop_point.y,
                    schema.default_size.width,
                    schema.default_size.height,
                ),
                anchor: AnchorSet::default(),
            },
            ContainerRule::Grid => Placement::Cell {
                cell: self.next_free_cell(parent),
                width: SizeMode::Fill,
            },
            ContainerRule::Flow | ContainerRule::Leaf => Placement::flow(),
        }
    }

    fn container_rule_of(&self, parent: NodeId) -> ContainerRule {
        let Ok(node) = self.document.node(parent) else {
            return ContainerRule::Leaf;
        };
        match self.catalog.schema(node.kind.as_str()) {
            Some(schema) => schema.container,
            None => ContainerRule::Leaf,
        }
    }

    /// The cell after the last occupied one, filling rows left to right.
    fn next_free_cell(&self, grid: NodeId) -> GridCell {
        let columns = self.grid_columns(grid);
        let mut next_slot: u32 = 0;
        for child in self.document.children_of(grid) {
            if let Ok(node) = self.document.node(*child)
                && let Placement::Cell { cell, .. } = &node.placement
            {
                let slot = cell.row * columns + cell.column + cell.column_span;
                next_slot = next_slot.max(slot);
            }
        }
        GridCell::new(next_slot / columns, next_slot % columns)
    }

    fn grid_columns(&self, grid: NodeId) -> u32 {
        let mut columns = GridSchema::DEFAULT_COLUMNS;
        if let Ok(node) = self.document.node(grid)
            && let Some(value) = node.properties.get("columns")
            && let Some(number) = value.as_integer()
        {
            columns = number;
        }
        u32::try_from(columns).unwrap_or(1).max(1)
    }
}

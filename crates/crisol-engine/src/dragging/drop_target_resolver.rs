use crisol_catalog::WidgetCatalog;
use crisol_catalog::schema::ContainerRule;
use crisol_model::document::{DocumentWalker, FormDocument};
use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;

use super::DropTarget;
use crate::layout::{HitTester, LayoutSnapshot};

/// Finds the container under the pointer and the position a dropped control takes in it.
pub struct DropTargetResolver<'context> {
    catalog: &'context WidgetCatalog,
    document: &'context FormDocument,
    snapshot: &'context LayoutSnapshot,
}

impl<'context> DropTargetResolver<'context> {
    const MARKER_THICKNESS: i32 = 2;

    pub fn new(
        catalog: &'context WidgetCatalog,
        document: &'context FormDocument,
        snapshot: &'context LayoutSnapshot,
    ) -> Self {
        Self {
            catalog,
            document,
            snapshot,
        }
    }

    /// The drop target at `pointer`, ignoring `dragged` and everything inside it.
    pub fn resolve(&self, pointer: Point, dragged: Option<NodeId>) -> DropTarget {
        let container = self.container_at(pointer, dragged);
        let container_rect = self.snapshot.bounds_of(container).unwrap_or_default();
        let local_point = container_rect.origin.distance_to(pointer);
        let rule = self.rule_of(container);
        let siblings = self.sibling_rects(container, dragged);
        let index = match rule {
            ContainerRule::Absolute | ContainerRule::Leaf => {
                self.document.children_of(container).len()
            }
            ContainerRule::Flow if self.is_horizontal(container) => {
                Self::count_before(&siblings, |rect| rect.center_x() < pointer.x)
            }
            ContainerRule::Flow => {
                Self::count_before(&siblings, |rect| rect.center_y() < pointer.y)
            }
            ContainerRule::Grid => Self::count_before(&siblings, |rect| {
                rect.bottom() <= pointer.y
                    || (rect.top() <= pointer.y && rect.center_x() < pointer.x)
            }),
        };
        let marker = match rule {
            ContainerRule::Flow | ContainerRule::Grid => Some(Self::marker_for(
                container_rect,
                &siblings,
                index,
                self.is_horizontal(container),
            )),
            _ => None,
        };
        DropTarget {
            container,
            index,
            local_point,
            marker,
        }
    }

    fn container_at(&self, pointer: Point, dragged: Option<NodeId>) -> NodeId {
        let walker = DocumentWalker::new(self.document);
        for node in HitTester::new(self.snapshot, self.document).stack_at(pointer) {
            if let Some(moving) = dragged
                && walker.is_within(node, moving)
            {
                continue;
            }
            if self.rule_of(node).accepts_children() {
                return node;
            }
        }
        self.document.root_id()
    }

    fn rule_of(&self, node: NodeId) -> ContainerRule {
        let Ok(found) = self.document.node(node) else {
            return ContainerRule::Leaf;
        };
        match self.catalog.schema(found.kind.as_str()) {
            Some(schema) => schema.container,
            None => ContainerRule::Leaf,
        }
    }

    fn is_horizontal(&self, container: NodeId) -> bool {
        let Ok(node) = self.document.node(container) else {
            return false;
        };
        node.properties
            .get("direction")
            .and_then(|value| value.as_text())
            == Some("horizontal")
    }

    fn sibling_rects(&self, container: NodeId, dragged: Option<NodeId>) -> Vec<Rect> {
        let mut rects = Vec::new();
        for child in self.document.children_of(container) {
            if Some(*child) == dragged {
                continue;
            }
            if let Some(rect) = self.snapshot.bounds_of(*child) {
                rects.push(rect);
            }
        }
        rects
    }

    fn count_before(siblings: &[Rect], is_before: impl Fn(&Rect) -> bool) -> usize {
        let mut count = 0;
        for rect in siblings {
            if is_before(rect) {
                count += 1;
            }
        }
        count
    }

    fn marker_for(container: Rect, siblings: &[Rect], index: usize, horizontal: bool) -> Rect {
        let thickness = Self::MARKER_THICKNESS;
        let reference = siblings.get(index).or_else(|| siblings.last());
        match (reference, horizontal) {
            (None, _) => Rect::new(
                container.left(),
                container.top(),
                container.size.width,
                thickness,
            ),
            (Some(rect), false) => {
                let y = if index < siblings.len() {
                    rect.top()
                } else {
                    rect.bottom()
                };
                Rect::new(rect.left(), y - thickness, rect.size.width, thickness)
            }
            (Some(rect), true) => {
                let x = if index < siblings.len() {
                    rect.left()
                } else {
                    rect.right()
                };
                Rect::new(x - thickness, rect.top(), thickness, rect.size.height)
            }
        }
    }
}

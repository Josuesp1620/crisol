use crisol_catalog::naming::NodeFactory;
use crisol_commands::Command;
use crisol_commands::commands::{MoveNodeCommand, SetPlacementCommand, Transaction};
use crisol_engine::EngineSettings;
use crisol_engine::dragging::{DragKind, DragSession, DropTargetResolver};
use crisol_engine::layout::{HitTester, LayoutSnapshot};
use crisol_engine::selection::ResizeHandle;
use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;
use crisol_model::layout::{Placement, SizeMode};
use gpui::{Context, Modifiers, Pixels};

use super::{CanvasInteraction, DesignerWorkspace};

/// What the pointer does on the canvas: select, move, resize and drop from the toolbox.
impl DesignerWorkspace {
    /// Pixels the pointer must travel before a press becomes a drag.
    const DRAG_THRESHOLD: i32 = 3;

    pub fn canvas_mouse_down(
        &mut self,
        position: gpui::Point<Pixels>,
        modifiers: Modifiers,
        cx: &mut Context<Self>,
    ) {
        let point = Self::point_of(position);
        let snapshot = self.session().recorder().snapshot();
        let settings = EngineSettings::default();
        self.open_menu = None;
        if let Some(selected) = self.session().selection().primary()
            && let Some(rect) = snapshot.bounds_of(selected)
            && let Some(handle) = ResizeHandle::at(rect, point, settings.handle_size)
        {
            self.interaction = CanvasInteraction::Pressing {
                node: selected,
                kind: DragKind::Resize(handle),
                start: point,
                rect,
            };
            return;
        }
        let document = self.session().document();
        let hit = HitTester::new(&snapshot, document).topmost_at(point);
        let Some(node) = hit else {
            let root = document.root_id();
            self.select(root, false, cx);
            self.interaction = CanvasInteraction::Idle;
            return;
        };
        self.select(node, modifiers.control || modifiers.platform, cx);
        if let Some(rect) = snapshot.bounds_of(node) {
            self.interaction = CanvasInteraction::Pressing {
                node,
                kind: DragKind::Move,
                start: point,
                rect,
            };
        }
    }

    pub fn canvas_mouse_move(&mut self, position: gpui::Point<Pixels>, cx: &mut Context<Self>) {
        let point = Self::point_of(position);
        let interaction = std::mem::take(&mut self.interaction);
        self.interaction = match interaction {
            CanvasInteraction::Pressing {
                node,
                kind,
                start,
                rect,
            } => {
                let moved = (point.x - start.x).abs() > Self::DRAG_THRESHOLD
                    || (point.y - start.y).abs() > Self::DRAG_THRESHOLD;
                if moved {
                    let session =
                        DragSession::begin(node, kind, start, rect, EngineSettings::default());
                    self.dragging(session, point)
                } else {
                    CanvasInteraction::Pressing {
                        node,
                        kind,
                        start,
                        rect,
                    }
                }
            }
            CanvasInteraction::Dragging { session, .. } => self.dragging(session, point),
            CanvasInteraction::Idle => CanvasInteraction::Idle,
        };
        cx.notify();
    }

    pub fn canvas_mouse_up(&mut self, cx: &mut Context<Self>) {
        let interaction = std::mem::take(&mut self.interaction);
        if let CanvasInteraction::Dragging {
            session,
            preview,
            drop,
        } = interaction
        {
            let command = self.finish_drag(&session, preview.rect, drop);
            if let Some(command) = command {
                self.execute(command.as_ref(), cx);
            }
        }
        cx.notify();
    }

    pub fn toolbox_drag_over(&mut self, position: gpui::Point<Pixels>, cx: &mut Context<Self>) {
        let snapshot = self.session().recorder().snapshot();
        let resolver = DropTargetResolver::new(&self.catalog, self.session().document(), &snapshot);
        self.toolbox_hover = Some(resolver.resolve(Self::point_of(position), None));
        cx.notify();
    }

    pub fn toolbox_drop(
        &mut self,
        kind: &str,
        position: gpui::Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.session().recorder().snapshot();
        let resolver = DropTargetResolver::new(&self.catalog, self.session().document(), &snapshot);
        let target = resolver.resolve(Self::point_of(position), None);
        self.toolbox_hover = None;
        self.add_control(kind, target, cx);
    }

    fn dragging(&self, session: DragSession, point: Point) -> CanvasInteraction {
        let snapshot = self.session().recorder().snapshot();
        let references = self.references_for(session.node, &snapshot);
        let preview = session.preview(point, &references);
        let drop = if session.kind == DragKind::Move && !self.is_absolute(session.node) {
            let resolver =
                DropTargetResolver::new(&self.catalog, self.session().document(), &snapshot);
            Some(resolver.resolve(point, Some(session.node)))
        } else {
            None
        };
        CanvasInteraction::Dragging {
            session,
            preview,
            drop,
        }
    }

    fn finish_drag(
        &self,
        session: &DragSession,
        final_rect: Rect,
        drop: Option<crisol_engine::dragging::DropTarget>,
    ) -> Option<Box<dyn Command>> {
        let document = self.session().document();
        let snapshot = self.session().recorder().snapshot();
        let parent = document.parent_of(session.node)?;
        let parent_origin = snapshot
            .bounds_of(parent)
            .map(|rect| rect.origin)
            .unwrap_or_default();
        if let Some(command) = session.finish(final_rect, parent_origin, document) {
            return Some(command);
        }
        match (session.kind, drop) {
            (DragKind::Move, Some(target)) => self.move_command(session.node, target),
            (DragKind::Resize(_), _) => self.resize_command(session.node, final_rect),
            _ => None,
        }
    }

    fn move_command(
        &self,
        node: NodeId,
        target: crisol_engine::dragging::DropTarget,
    ) -> Option<Box<dyn Command>> {
        let document = self.session().document();
        let same_place = document.parent_of(node) == Some(target.container)
            && document.index_in_parent(node) == Some(target.index);
        if same_place {
            return None;
        }
        let moving = MoveNodeCommand {
            node,
            new_parent: target.container,
            index: target.index,
        };
        if document.parent_of(node) == Some(target.container) {
            return Some(Box::new(moving));
        }
        let kind = document.node(node).ok()?.kind.clone();
        let schema = self.catalog.schema(kind.as_str())?;
        let placement = NodeFactory::new(&self.catalog, document).placement_for(
            schema,
            target.container,
            target.local_point,
        );
        let place = SetPlacementCommand { node, placement };
        Some(Box::new(Transaction::new(
            "Move control",
            vec![Box::new(moving), Box::new(place)],
        )))
    }

    fn resize_command(&self, node: NodeId, rect: Rect) -> Option<Box<dyn Command>> {
        let placement = match &self.session().document().node(node).ok()?.placement {
            Placement::Flow { .. } => Placement::Flow {
                width: SizeMode::Fixed(rect.size.width),
                height: SizeMode::Fixed(rect.size.height),
            },
            Placement::Cell { cell, .. } => Placement::Cell {
                cell: *cell,
                width: SizeMode::Fixed(rect.size.width),
            },
            Placement::Absolute { .. } => return None,
        };
        Some(Box::new(SetPlacementCommand { node, placement }))
    }

    /// Rectangles a dragged control can align with: its siblings and its container.
    fn references_for(&self, node: NodeId, snapshot: &LayoutSnapshot) -> Vec<Rect> {
        let document = self.session().document();
        let mut references = Vec::new();
        let Some(parent) = document.parent_of(node) else {
            return references;
        };
        if let Some(rect) = snapshot.bounds_of(parent) {
            references.push(rect);
        }
        for sibling in document.children_of(parent) {
            if *sibling != node
                && let Some(rect) = snapshot.bounds_of(*sibling)
            {
                references.push(rect);
            }
        }
        references
    }

    fn is_absolute(&self, node: NodeId) -> bool {
        self.session()
            .document()
            .node(node)
            .is_ok_and(|found| matches!(found.placement, Placement::Absolute { .. }))
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "window coordinates fit in i32"
    )]
    fn point_of(position: gpui::Point<Pixels>) -> Point {
        Point::new(
            f32::from(position.x).round() as i32,
            f32::from(position.y).round() as i32,
        )
    }
}

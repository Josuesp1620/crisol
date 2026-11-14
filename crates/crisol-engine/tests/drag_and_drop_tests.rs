use crisol_catalog::WidgetCatalog;
use crisol_catalog::naming::NodeFactory;
use crisol_engine::EngineSettings;
use crisol_engine::dragging::{DragKind, DragSession, DropTargetResolver};
use crisol_engine::layout::{HitTester, LayoutSnapshot};
use crisol_engine::selection::ResizeHandle;
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Point, Rect, Size};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::{Placement, SizeMode};

struct Scene {
    catalog: WidgetCatalog,
    document: FormDocument,
    snapshot: LayoutSnapshot,
}

impl Scene {
    fn new() -> Self {
        let form = FormInfo {
            name: TypeName::parse("CustomerForm").unwrap(),
            title: "New customer".to_owned(),
            size: Size::new(400, 300),
            state: None,
        };
        let root = Node::new(
            WidgetKind::new("Form"),
            Identifier::parse("frm_customer").unwrap(),
            Placement::flow(),
        );
        let mut snapshot = LayoutSnapshot::default();
        snapshot.record(root.id, Rect::new(0, 0, 400, 300));
        Self {
            catalog: WidgetCatalog::standard(),
            document: FormDocument::new(form, root),
            snapshot,
        }
    }

    fn add(&mut self, kind: &str, parent: NodeId, bounds: Rect) -> NodeId {
        let schema = self.catalog.schema(kind).unwrap();
        let node = NodeFactory::new(&self.catalog, &self.document).create(
            schema,
            parent,
            Point::new(bounds.left(), bounds.top()),
        );
        let id = node.id;
        let index = self.document.children_of(parent).len();
        self.document.insert(parent, index, node).unwrap();
        self.snapshot.record(id, bounds);
        id
    }
}

#[test]
fn the_topmost_control_wins_and_the_root_is_never_hit() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    let panel = scene.add("Panel", root, Rect::new(0, 0, 300, 200));
    let button = scene.add("Button", panel, Rect::new(20, 20, 100, 32));
    let tester = HitTester::new(&scene.snapshot, &scene.document);
    assert_eq!(tester.stack_at(Point::new(30, 30)), vec![button, panel]);
    assert_eq!(tester.topmost_at(Point::new(350, 250)), None);
}

#[test]
fn dropping_between_stack_children_picks_the_index() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    scene.add("Label", root, Rect::new(0, 0, 400, 20));
    scene.add("Label", root, Rect::new(0, 30, 400, 20));
    let resolver = DropTargetResolver::new(&scene.catalog, &scene.document, &scene.snapshot);
    let target = resolver.resolve(Point::new(50, 28), None);
    assert_eq!((target.container, target.index), (root, 1));
    assert_eq!(target.marker, Some(Rect::new(0, 28, 400, 2)));
}

#[test]
fn dropping_on_a_panel_uses_local_coordinates() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    let panel = scene.add("Panel", root, Rect::new(10, 10, 300, 200));
    let resolver = DropTargetResolver::new(&scene.catalog, &scene.document, &scene.snapshot);
    let target = resolver.resolve(Point::new(60, 70), None);
    assert_eq!(target.container, panel);
    assert_eq!(target.local_point, Point::new(50, 60));
    assert_eq!(target.marker, None);
}

#[test]
fn moving_on_a_panel_produces_one_placement_command_in_panel_coordinates() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    let panel = scene.add("Panel", root, Rect::new(10, 10, 300, 200));
    let button = scene.add("Button", panel, Rect::new(30, 30, 100, 32));
    let session = DragSession::begin(
        button,
        DragKind::Move,
        Point::new(40, 40),
        Rect::new(30, 30, 100, 32),
        EngineSettings::default(),
    );
    let preview = session.preview(Point::new(90, 75), &[]);
    assert_eq!(preview.rect, Rect::new(80, 65, 100, 32));
    let command = session
        .finish(preview.rect, Point::new(10, 10), &scene.document)
        .unwrap();
    command.apply(&mut scene.document).unwrap();
    match &scene.document.node(button).unwrap().placement {
        Placement::Absolute { rect, .. } => assert_eq!(*rect, Rect::new(70, 55, 100, 32)),
        other => panic!("unexpected placement {other:?}"),
    }
}

#[test]
fn resizing_never_goes_below_the_minimum_size() {
    let session = DragSession::begin(
        NodeId::generate(),
        DragKind::Resize(ResizeHandle::BottomRight),
        Point::new(100, 100),
        Rect::new(0, 0, 100, 100),
        EngineSettings::default(),
    );
    let preview = session.preview(Point::new(-50, -50), &[]);
    assert_eq!(preview.rect.size, Size::new(8, 8));
}

#[test]
fn flow_controls_are_reordered_not_placed() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    let label = scene.add("Label", root, Rect::new(0, 0, 100, 20));
    assert!(matches!(
        scene.document.node(label).unwrap().placement,
        Placement::Flow {
            width: SizeMode::Hug,
            ..
        }
    ));
    let session = DragSession::begin(
        label,
        DragKind::Move,
        Point::new(5, 5),
        Rect::new(0, 0, 100, 20),
        EngineSettings::default(),
    );
    assert!(
        session
            .finish(Rect::new(0, 50, 100, 20), Point::new(0, 0), &scene.document)
            .is_none()
    );
}

#[test]
fn children_win_over_their_container_whatever_the_recording_order() {
    let mut scene = Scene::new();
    let root = scene.document.root_id();
    let panel = scene.add("Panel", root, Rect::new(0, 0, 300, 200));
    let button = scene.add("Button", panel, Rect::new(20, 20, 100, 32));
    let mut reversed = LayoutSnapshot::default();
    reversed.record(button, Rect::new(20, 20, 100, 32));
    reversed.record(panel, Rect::new(0, 0, 300, 200));
    let tester = HitTester::new(&reversed, &scene.document);
    assert_eq!(tester.topmost_at(Point::new(30, 30)), Some(button));
}

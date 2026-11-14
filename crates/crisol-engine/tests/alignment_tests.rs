use crisol_commands::Command;
use crisol_engine::arranging::{Alignment, AlignmentTool};
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Rect, Size};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::{AnchorSet, Placement};

struct Panel {
    document: FormDocument,
    controls: Vec<NodeId>,
}

impl Panel {
    fn with(rects: &[Rect]) -> Self {
        let form = FormInfo {
            name: TypeName::parse("AlignForm").unwrap(),
            title: "Align".to_owned(),
            size: Size::new(400, 300),
            state: None,
        };
        let mut document = FormDocument::new(
            form,
            Node::new(
                WidgetKind::new("Panel"),
                Identifier::parse("pnl_root").unwrap(),
                Placement::flow(),
            ),
        );
        let root = document.root_id();
        let mut controls = Vec::new();
        for (index, rect) in rects.iter().enumerate() {
            let node = Node::new(
                WidgetKind::new("Button"),
                Identifier::parse(&format!("btn_{index}")).unwrap(),
                Placement::Absolute {
                    rect: *rect,
                    anchor: AnchorSet::default(),
                },
            );
            controls.push(node.id);
            document.insert(root, index, node).unwrap();
        }
        Self { document, controls }
    }

    fn arrange(&mut self, alignment: Alignment) {
        let transaction = AlignmentTool::new(&self.document)
            .arrange(&self.controls, alignment)
            .unwrap();
        transaction.apply(&mut self.document).unwrap();
    }

    fn rect(&self, index: usize) -> Rect {
        match &self.document.node(self.controls[index]).unwrap().placement {
            Placement::Absolute { rect, .. } => *rect,
            other => panic!("unexpected placement {other:?}"),
        }
    }
}

#[test]
fn left_edges_follow_the_primary_control() {
    let mut panel = Panel::with(&[Rect::new(20, 10, 100, 30), Rect::new(57, 60, 80, 30)]);
    panel.arrange(Alignment::Left);
    assert_eq!(panel.rect(1), Rect::new(20, 60, 80, 30));
}

#[test]
fn bottoms_and_sizes_follow_the_primary_control() {
    let mut panel = Panel::with(&[Rect::new(20, 10, 100, 30), Rect::new(150, 70, 80, 20)]);
    panel.arrange(Alignment::Bottom);
    assert_eq!(panel.rect(1), Rect::new(150, 20, 80, 20));
    panel.arrange(Alignment::SameSize);
    assert_eq!(panel.rect(1).size, Size::new(100, 30));
}

#[test]
fn spacing_leaves_equal_gaps() {
    let mut panel = Panel::with(&[
        Rect::new(0, 0, 50, 20),
        Rect::new(70, 0, 50, 20),
        Rect::new(250, 0, 50, 20),
    ]);
    panel.arrange(Alignment::SpaceHorizontally);
    assert_eq!(panel.rect(1).left(), 125);
}

#[test]
fn a_single_control_cannot_be_aligned() {
    let panel = Panel::with(&[Rect::new(0, 0, 50, 20)]);
    assert!(!AlignmentTool::new(&panel.document).can_align(&panel.controls));
}

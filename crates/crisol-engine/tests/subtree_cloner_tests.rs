use crisol_commands::Command;
use crisol_engine::editing::SubtreeCloner;
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Rect, Size};
use crisol_model::identity::{Identifier, TypeName};
use crisol_model::layout::{AnchorSet, Placement};

fn node(kind: &str, name: &str, placement: Placement) -> Node {
    Node::new(
        WidgetKind::new(kind),
        Identifier::parse(name).unwrap(),
        placement,
    )
}

fn form() -> FormDocument {
    let info = FormInfo {
        name: TypeName::parse("CopyForm").unwrap(),
        title: "Copy".to_owned(),
        size: Size::new(400, 300),
        state: None,
    };
    FormDocument::new(info, node("Form", "frm_copy", Placement::flow()))
}

#[test]
fn pasting_a_container_copies_its_children_with_unique_names() {
    let mut document = form();
    let root = document.root_id();
    let stack = node("Stack", "stk_buttons", Placement::flow());
    let stack_id = stack.id;
    document.insert(root, 0, stack).unwrap();
    document
        .insert(stack_id, 0, node("Button", "btn_save", Placement::flow()))
        .unwrap();
    let origin = document.clone();
    let (transaction, roots) = SubtreeCloner::new(&document)
        .paste(&origin, &[stack_id], root)
        .unwrap();
    transaction.apply(&mut document).unwrap();
    assert_eq!(roots.len(), 1);
    assert_eq!(document.children_of(root).len(), 2);
    let copy = document.node(roots[0]).unwrap();
    assert_eq!(copy.name.as_str(), "stk_buttons_2");
    let child = document.node(copy.children()[0]).unwrap();
    assert_eq!(child.name.as_str(), "btn_save_2");
}

#[test]
fn pasted_absolute_controls_move_aside() {
    let mut document = form();
    let root = document.root_id();
    let button = node(
        "Button",
        "btn_ok",
        Placement::Absolute {
            rect: Rect::new(10, 10, 80, 30),
            anchor: AnchorSet::default(),
        },
    );
    let button_id = button.id;
    document.insert(root, 0, button).unwrap();
    let origin = document.clone();
    let mut cloner = SubtreeCloner::new(&document);
    let (transaction, roots) = cloner
        .paste(&origin, &[button_id, button_id], root)
        .unwrap();
    transaction.apply(&mut document).unwrap();
    let first = document.node(roots[0]).unwrap();
    assert!(
        matches!(first.placement, Placement::Absolute { rect, .. } if rect == Rect::new(26, 26, 80, 30))
    );
    assert_eq!(document.node(roots[1]).unwrap().name.as_str(), "btn_ok_3");
}

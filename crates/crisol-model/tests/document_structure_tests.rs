mod support;

use crisol_model::ModelError;
use crisol_model::document::DocumentWalker;
use support::SampleForm;

#[test]
fn inserted_nodes_keep_their_order() {
    let sample = SampleForm::build();
    let root = sample.document.root_id();
    assert_eq!(
        sample.document.children_of(root),
        &[sample.text_box, sample.button]
    );
    assert_eq!(sample.document.parent_of(sample.button), Some(root));
}

#[test]
fn names_are_unique_in_a_form() {
    let mut sample = SampleForm::build();
    let root = sample.document.root_id();
    let duplicate = SampleForm::node("Button", "btn_save");
    let result = sample.document.insert(root, 0, duplicate);
    assert!(matches!(result, Err(ModelError::DuplicateName(_))));
}

#[test]
fn detaching_and_reattaching_restores_the_tree() {
    let mut sample = SampleForm::build();
    let before = sample.document.clone();
    let subtree = sample.document.detach(sample.text_box).unwrap();
    assert!(!sample.document.contains(sample.text_box));
    sample.document.reattach(subtree).unwrap();
    assert_eq!(sample.document, before);
}

#[test]
fn moving_a_node_reports_where_it_was() {
    let mut sample = SampleForm::build();
    let root = sample.document.root_id();
    let (old_parent, old_index) = sample.document.move_node(sample.button, root, 0).unwrap();
    assert_eq!((old_parent, old_index), (root, 1));
    assert_eq!(
        sample.document.children_of(root),
        &[sample.button, sample.text_box]
    );
}

#[test]
fn a_node_cannot_move_inside_itself() {
    let mut sample = SampleForm::build();
    let container = SampleForm::node("Stack", "stk_inner");
    let container_id = container.id;
    let root = sample.document.root_id();
    sample.document.insert(root, 0, container).unwrap();
    let inner = SampleForm::node("Label", "lbl_inside");
    sample.document.insert(container_id, 0, inner).unwrap();
    let inner_id = sample.document.find_by_name("lbl_inside").unwrap();
    let result = sample.document.move_node(container_id, inner_id, 0);
    assert!(matches!(result, Err(ModelError::CyclicMove)));
}

#[test]
fn the_root_cannot_be_detached() {
    let mut sample = SampleForm::build();
    let root = sample.document.root_id();
    assert!(matches!(
        sample.document.detach(root),
        Err(ModelError::RootIsFixed)
    ));
}

#[test]
fn the_walker_visits_parents_before_children() {
    let sample = SampleForm::build();
    let order = DocumentWalker::new(&sample.document).all();
    assert_eq!(
        order,
        vec![sample.document.root_id(), sample.text_box, sample.button]
    );
}

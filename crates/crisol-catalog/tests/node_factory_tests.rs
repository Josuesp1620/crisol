mod support;

use crisol_model::geometry::Rect;
use crisol_model::layout::{GridCell, Placement};
use support::SampleDocument;

#[test]
fn new_controls_get_the_first_free_numbered_name() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let first = sample.add("Button", root);
    let second = sample.add("Button", root);
    assert_eq!(sample.name_of(first), "btn_1");
    assert_eq!(sample.name_of(second), "btn_2");
}

#[test]
fn grid_children_fill_rows_left_to_right() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let grid = sample.add("Grid", root);
    let mut cells = Vec::new();
    for _ in 0..3 {
        let child = sample.add("Label", grid);
        if let Placement::Cell { cell, .. } = sample.document.node(child).unwrap().placement {
            cells.push(cell);
        }
    }
    assert_eq!(
        cells,
        vec![
            GridCell::new(0, 0),
            GridCell::new(0, 1),
            GridCell::new(1, 0)
        ]
    );
}

#[test]
fn absolute_panels_place_children_at_the_drop_point_with_the_default_size() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let panel = sample.add("Panel", root);
    let button = sample.add("Button", panel);
    match &sample.document.node(button).unwrap().placement {
        Placement::Absolute { rect, .. } => assert_eq!(*rect, Rect::new(16, 24, 100, 32)),
        other => panic!("unexpected placement {other:?}"),
    }
}

#[test]
fn stacks_place_children_in_flow() {
    let mut sample = SampleDocument::empty();
    let root = sample.root();
    let label = sample.add("Label", root);
    assert!(matches!(
        sample.document.node(label).unwrap().placement,
        Placement::Flow { .. }
    ));
}

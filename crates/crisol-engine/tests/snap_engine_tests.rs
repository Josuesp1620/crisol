use crisol_engine::selection::ResizeHandle;
use crisol_engine::snapping::{GuideAxis, SnapEngine};
use crisol_model::geometry::Rect;

#[test]
fn a_close_left_edge_snaps_and_shows_a_vertical_guide() {
    let engine = SnapEngine::new(6);
    let neighbour = Rect::new(100, 20, 120, 30);
    let result = engine.snap_move(Rect::new(104, 80, 120, 30), &[neighbour]);
    assert_eq!(result.rect, Rect::new(100, 80, 120, 30));
    assert_eq!(result.guides[0].axis, GuideAxis::Vertical);
    assert_eq!(result.guides[0].position, 100);
    assert_eq!((result.guides[0].start, result.guides[0].end), (20, 110));
}

#[test]
fn far_rectangles_do_not_snap() {
    let engine = SnapEngine::new(6);
    let moving = Rect::new(150, 200, 40, 40);
    let result = engine.snap_move(moving, &[Rect::new(0, 0, 50, 50)]);
    assert_eq!(result.rect, moving);
    assert!(result.guides.is_empty());
}

#[test]
fn centers_align_too() {
    let engine = SnapEngine::new(6);
    let result = engine.snap_move(Rect::new(0, 103, 40, 20), &[Rect::new(200, 100, 40, 30)]);
    assert_eq!(result.rect.center_y(), 115);
}

#[test]
fn resizing_moves_only_the_dragged_edge() {
    let engine = SnapEngine::new(6);
    let result = engine.snap_resize(
        Rect::new(10, 10, 95, 30),
        ResizeHandle::Right,
        &[Rect::new(40, 100, 60, 20)],
    );
    assert_eq!(result.rect, Rect::new(10, 10, 90, 30));
}

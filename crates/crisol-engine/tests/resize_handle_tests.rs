use crisol_engine::selection::ResizeHandle;
use crisol_model::geometry::{Point, Rect};

#[test]
fn handles_sit_on_corners_and_edge_middles() {
    let rect = Rect::new(10, 20, 100, 40);
    assert_eq!(ResizeHandle::TopLeft.center_on(rect), Point::new(10, 20));
    assert_eq!(ResizeHandle::Bottom.center_on(rect), Point::new(60, 60));
    assert_eq!(ResizeHandle::Right.center_on(rect), Point::new(110, 40));
}

#[test]
fn the_handle_under_the_pointer_is_found() {
    let rect = Rect::new(10, 20, 100, 40);
    assert_eq!(
        ResizeHandle::at(rect, Point::new(111, 61), 7),
        Some(ResizeHandle::BottomRight)
    );
    assert_eq!(ResizeHandle::at(rect, Point::new(60, 40), 7), None);
}

use crisol_model::geometry::{Point, Rect};

use super::{GuideAxis, SnapGuide, SnapResult};
use crate::selection::ResizeHandle;

/// Aligns a moving or resizing rectangle with reference rectangles (siblings and parent).
pub struct SnapEngine {
    distance: i32,
}

/// The closest alignment found on one axis: how far to shift and where the guide goes.
struct AxisMatch {
    shift: i32,
    position: i32,
    reference: Rect,
}

impl SnapEngine {
    pub fn new(distance: i32) -> Self {
        Self { distance }
    }

    /// Shifts the whole rectangle so its edges or center meet the nearest reference line.
    pub fn snap_move(&self, moving: Rect, references: &[Rect]) -> SnapResult {
        let horizontal = self.closest(
            &[moving.left(), moving.center_x(), moving.right()],
            references,
            GuideAxis::Vertical,
        );
        let vertical = self.closest(
            &[moving.top(), moving.center_y(), moving.bottom()],
            references,
            GuideAxis::Horizontal,
        );
        let mut origin = moving.origin;
        if let Some(found) = &horizontal {
            origin.x += found.shift;
        }
        if let Some(found) = &vertical {
            origin.y += found.shift;
        }
        let rect = moving.moved_to(origin);
        SnapResult {
            rect,
            guides: Self::guides_for(rect, horizontal, vertical),
        }
    }

    /// Moves only the edges the handle drags so they meet the nearest reference line.
    pub fn snap_resize(
        &self,
        resizing: Rect,
        handle: ResizeHandle,
        references: &[Rect],
    ) -> SnapResult {
        let mut left = resizing.left();
        let mut right = resizing.right();
        let mut top = resizing.top();
        let mut bottom = resizing.bottom();
        let moving_x = if handle.moves_left_edge() {
            Some(left)
        } else if handle.moves_right_edge() {
            Some(right)
        } else {
            None
        };
        let moving_y = if handle.moves_top_edge() {
            Some(top)
        } else if handle.moves_bottom_edge() {
            Some(bottom)
        } else {
            None
        };
        let horizontal =
            moving_x.and_then(|edge| self.closest(&[edge], references, GuideAxis::Vertical));
        let vertical =
            moving_y.and_then(|edge| self.closest(&[edge], references, GuideAxis::Horizontal));
        if let Some(found) = &horizontal {
            if handle.moves_left_edge() {
                left += found.shift;
            } else {
                right += found.shift;
            }
        }
        if let Some(found) = &vertical {
            if handle.moves_top_edge() {
                top += found.shift;
            } else {
                bottom += found.shift;
            }
        }
        let rect = Rect::new(left, top, right - left, bottom - top);
        SnapResult {
            rect,
            guides: Self::guides_for(rect, horizontal, vertical),
        }
    }

    fn closest(
        &self,
        candidates: &[i32],
        references: &[Rect],
        axis: GuideAxis,
    ) -> Option<AxisMatch> {
        let mut best: Option<AxisMatch> = None;
        for reference in references {
            let lines = match axis {
                GuideAxis::Vertical => [reference.left(), reference.center_x(), reference.right()],
                GuideAxis::Horizontal => {
                    [reference.top(), reference.center_y(), reference.bottom()]
                }
            };
            for line in lines {
                for candidate in candidates {
                    let shift = line - candidate;
                    let closer = best
                        .as_ref()
                        .is_none_or(|current| shift.abs() < current.shift.abs());
                    if shift.abs() <= self.distance && closer {
                        best = Some(AxisMatch {
                            shift,
                            position: line,
                            reference: *reference,
                        });
                    }
                }
            }
        }
        best
    }

    fn guides_for(
        rect: Rect,
        horizontal: Option<AxisMatch>,
        vertical: Option<AxisMatch>,
    ) -> Vec<SnapGuide> {
        let mut guides = Vec::new();
        if let Some(found) = horizontal {
            guides.push(SnapGuide {
                axis: GuideAxis::Vertical,
                position: found.position,
                start: rect.top().min(found.reference.top()),
                end: rect.bottom().max(found.reference.bottom()),
            });
        }
        if let Some(found) = vertical {
            guides.push(SnapGuide {
                axis: GuideAxis::Horizontal,
                position: found.position,
                start: rect.left().min(found.reference.left()),
                end: rect.right().max(found.reference.right()),
            });
        }
        guides
    }

    /// The point where a guide line starts, for drawing.
    pub fn guide_origin(guide: &SnapGuide) -> Point {
        match guide.axis {
            GuideAxis::Vertical => Point::new(guide.position, guide.start),
            GuideAxis::Horizontal => Point::new(guide.start, guide.position),
        }
    }
}

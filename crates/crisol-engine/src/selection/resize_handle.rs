use crisol_model::geometry::{Point, Rect};

/// One of the eight squares around a selected control that resize it when dragged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl ResizeHandle {
    pub const ALL: [ResizeHandle; 8] = [
        ResizeHandle::TopLeft,
        ResizeHandle::Top,
        ResizeHandle::TopRight,
        ResizeHandle::Right,
        ResizeHandle::BottomRight,
        ResizeHandle::Bottom,
        ResizeHandle::BottomLeft,
        ResizeHandle::Left,
    ];

    /// Center of the handle on the edge of `rect`.
    pub fn center_on(self, rect: Rect) -> Point {
        let x = match self {
            ResizeHandle::TopLeft | ResizeHandle::Left | ResizeHandle::BottomLeft => rect.left(),
            ResizeHandle::Top | ResizeHandle::Bottom => rect.center_x(),
            ResizeHandle::TopRight | ResizeHandle::Right | ResizeHandle::BottomRight => {
                rect.right()
            }
        };
        let y = match self {
            ResizeHandle::TopLeft | ResizeHandle::Top | ResizeHandle::TopRight => rect.top(),
            ResizeHandle::Left | ResizeHandle::Right => rect.center_y(),
            ResizeHandle::BottomLeft | ResizeHandle::Bottom | ResizeHandle::BottomRight => {
                rect.bottom()
            }
        };
        Point::new(x, y)
    }

    /// The handle of `rect` under `point`, if any.
    pub fn at(rect: Rect, point: Point, handle_size: i32) -> Option<ResizeHandle> {
        for handle in Self::ALL {
            let center = handle.center_on(rect);
            let half = handle_size / 2 + 1;
            let area = Rect::new(center.x - half, center.y - half, half * 2, half * 2);
            if area.contains(point) {
                return Some(handle);
            }
        }
        None
    }

    pub fn moves_left_edge(self) -> bool {
        matches!(
            self,
            ResizeHandle::TopLeft | ResizeHandle::Left | ResizeHandle::BottomLeft
        )
    }

    pub fn moves_right_edge(self) -> bool {
        matches!(
            self,
            ResizeHandle::TopRight | ResizeHandle::Right | ResizeHandle::BottomRight
        )
    }

    pub fn moves_top_edge(self) -> bool {
        matches!(
            self,
            ResizeHandle::TopLeft | ResizeHandle::Top | ResizeHandle::TopRight
        )
    }

    pub fn moves_bottom_edge(self) -> bool {
        matches!(
            self,
            ResizeHandle::BottomLeft | ResizeHandle::Bottom | ResizeHandle::BottomRight
        )
    }
}

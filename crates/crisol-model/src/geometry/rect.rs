use serde::{Deserialize, Serialize};

use super::{Point, Size};

/// A rectangle in logical pixels, from its top-left corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn left(&self) -> i32 {
        self.origin.x
    }

    pub fn top(&self) -> i32 {
        self.origin.y
    }

    pub fn right(&self) -> i32 {
        self.origin.x + self.size.width
    }

    pub fn bottom(&self) -> i32 {
        self.origin.y + self.size.height
    }

    pub fn center_x(&self) -> i32 {
        self.origin.x + self.size.width / 2
    }

    pub fn center_y(&self) -> i32 {
        self.origin.y + self.size.height / 2
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left()
            && point.x < self.right()
            && point.y >= self.top()
            && point.y < self.bottom()
    }

    pub fn moved_to(&self, origin: Point) -> Self {
        Self {
            origin,
            size: self.size,
        }
    }

    pub fn resized_to(&self, size: Size) -> Self {
        Self {
            origin: self.origin,
            size,
        }
    }
}

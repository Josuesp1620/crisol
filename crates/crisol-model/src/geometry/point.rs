use serde::{Deserialize, Serialize};

/// A position in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn offset_by(self, delta: Point) -> Self {
        Self::new(self.x + delta.x, self.y + delta.y)
    }

    pub fn distance_to(self, other: Point) -> Point {
        Self::new(other.x - self.x, other.y - self.y)
    }
}

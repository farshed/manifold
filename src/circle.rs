use crate::Point;

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub const fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

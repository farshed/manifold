use crate::Point2;

pub struct Circle {
    pub center: Point2,
    pub radius: f32,
}

impl Circle {
    pub const fn new(center: Point2, radius: f32) -> Self {
        Self { center, radius }
    }
}

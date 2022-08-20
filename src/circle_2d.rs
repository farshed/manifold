use crate::Point2D;

pub struct Circle2D {
    pub center: Point2D,
    pub radius: f32,
}

impl Circle2D {
    pub const fn new(center: Point2D, radius: f32) -> Self {
        Self { center, radius }
    }
}

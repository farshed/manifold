use crate::Point2;

pub struct Circle2 {
    pub center: Point2,
    pub radius: f32,
}

impl Circle2 {
    pub const fn new(center: Point2, radius: f32) -> Self {
        Self { center, radius }
    }
}

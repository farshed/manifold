use crate::Point2;

pub struct Quad {
    pub x: Point2,
    pub y: Point2,
    pub z: Point2,
    pub w: Point2,
}

impl Quad {
    pub const fn new(x: Point2, y: Point2, z: Point2, w: Point2) -> Self {
        Self { x, y, z, w }
    }
}

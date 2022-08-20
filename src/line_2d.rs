use crate::{Point2D, Rect2D, Vector2D};

pub struct Line {
    pub origin: Point2D,
    pub direction: Vector2D,
}

impl Line {
    pub const fn new() {}
}

pub struct LineSeg {
    pub p0: Point2D,
    pub p1: Point2D,
}

impl LineSeg {
    #[inline]
    pub fn sample(&self, t: f32) -> Point2D {
        self.p0.lerp(self.p1, t)
    }

    #[inline]
    pub fn bounding_rect(&self) -> Rect2D {
        Rect2D::new(
            self.p0.x.min(self.p1.x),
            self.p0.y.min(self.p1.y),
            self.p0.x.max(self.p1.x),
            self.p0.y.max(self.p1.y),
        )
    }

    #[inline]
    pub fn to_vector(&self) -> Vector2D {
        self.p1 - self.p0
    }
}

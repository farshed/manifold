use crate::{Point2, Rect, Vec2};

pub struct Line {
    pub origin: Point2,
    pub direction: Vec2,
}

impl Line {
    pub const fn new() {}
}

pub struct LineSeg {
    pub p0: Point2,
    pub p1: Point2,
}

impl LineSeg {
    #[inline]
    pub fn sample(&self, t: f32) -> Point2 {
        self.p0.lerp(self.p1, t)
    }

    #[inline]
    pub fn bounding_rect(&self) -> Rect {
        Rect::new(
            self.p0.x.min(self.p1.x),
            self.p0.y.min(self.p1.y),
            self.p0.x.max(self.p1.x),
            self.p0.y.max(self.p1.y),
        )
    }

    #[inline]
    pub fn to_vector(&self) -> Vec2 {
        self.p1 - self.p0
    }
}

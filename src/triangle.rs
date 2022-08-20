use crate::Point2;

pub struct Triangle {
    pub p0: Point2,
    pub p1: Point2,
    pub p2: Point2,
}

impl Triangle {
    pub const fn new(p0: Point2, p1: Point2, p2: Point2) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn contains(&self, point: Point2) -> bool {
        let coords = self.to_barycentric_coords(point);
        coords.0 > 0. && coords.1 > 0. && coords.2 > 0.
    }

    fn to_barycentric_coords(&self, point: Point2) -> (f32, f32, f32) {
        let v0 = self.p1 - self.p0;
        let v1 = self.p2 - self.p0;
        let v2 = point - self.p0;
        let inv = 1. / v0.cross(v1);
        let a = v0.cross(v2) * inv;
        let b = v2.cross(v1) * inv;
        let c = 1. - a - b;
        (a, b, c)
    }
}

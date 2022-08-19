use crate::{Numeric, Point2};

pub struct Triangle<N: Numeric> {
    pub p0: Point2<N>,
    pub p1: Point2<N>,
    pub p2: Point2<N>,
}

impl<N: Numeric> Triangle<N> {
    pub const fn new(p0: Point2<N>, p1: Point2<N>, p2: Point2<N>) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn contains(&self, point: Point2<N>) -> bool {
        let coords = self.get_barycentric_coords_for_point(point);
        coords.0 > N::ZERO && coords.1 > N::ZERO && coords.2 > N::ZERO
    }

    fn to_barycentric_coords(&self, point: Point2<N>) -> (N, N, N) {
        let v0 = self.p1 - self.p0;
        let v1 = self.p2 - self.p0;
        let v2 = point - self.p0;
        let inv = N::ONE / v0.cross(v1);
        let a = v0.cross(v2) * inv;
        let b = v2.cross(v1) * inv;
        let c = N::ONE - a - b;
        (a, b, c)
    }
}

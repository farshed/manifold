use crate::Numeric;

pub struct Point2<N: Numeric> {
    pub x: N,
    pub y: N,
}

impl<N: Numeric> Point2<N> {
    pub const ORIGIN: Self = Self::splat(N::ZERO);

    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: N) -> Self {
        Self { x: v, y: v }
    }

    pub const fn extend(&self, z: N) -> Point3<N> {
        Point3::new(self.x, self.y, z)
    }

    pub const fn raw(&self) -> (N, N) {
        (self.x, self.y)
    }

    pub const fn to_3d(&self) -> Point3<N> {
        Point3::new(self.x, self.y, N::ZERO)
    }

    // pub fn to_vec2(&self) -> Vec2 {
    //     self.x.hypot(other)
    // }
}

pub struct Point3<N: Numeric> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Numeric> Point3<N> {
    pub const ORIGIN: Self = Self::splat(N::ZERO);

    pub const fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: N) -> Self {
        Self { x: v, y: v, z: v }
    }

    // pub fn to_vec2(&self) -> Vec2 {
    //     self.x.hypot(other)
    // }
}

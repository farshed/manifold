use crate::Point2;

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub const ORIGIN: Self = Self::splat(0.);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    pub const fn to_2d(&self) -> Point2 {
        Point2::new(self.x, self.y)
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        let one_t = 1. - t;
        Self::new(
            one_t * self.x + t * other.x,
            one_t * self.y + t * other.y,
            one_t * self.z + t * other.z,
        )
    }

    // pub fn to_vec2(&self) -> Vec2 {
    //     self.x.hypot(other)
    // }
}

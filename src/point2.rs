use crate::{Point3, Vector2};
use std::cmp::PartialOrd;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub};

/// A point in 2 space
#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Point2 {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}

impl Point2 {
    pub const ORIGIN: Self = Self::splat(0.);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub const fn extend(&self, z: f32) -> Point3 {
        Point3::new(self.x, self.y, z)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        let one_t = 1. - t;
        Self::new(one_t * self.x + t * other.x, one_t * self.y + t * other.y)
    }
}

impl From<(f32, f32)> for Point2 {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Point2 { x: v.0, y: v.1 }
    }
}

impl From<Vector2> for Point2 {
    #[inline]
    fn from(v: Vector2) -> Self {
        Point2 { x: v.x, y: v.y }
    }
}

impl Add<Vector2> for Point2 {
    type Output = Point2;

    #[inline]
    fn add(self, other: Vector2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Vector2> for Point2 {
    #[inline]
    fn add_assign(&mut self, other: Vector2) {
        *self = Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, other: Self) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Display for Point2 {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "𝛒=(")?;
        Display::fmt(&self.x, formatter)?;
        write!(formatter, ", ")?;
        Display::fmt(&self.y, formatter)?;
        write!(formatter, ")")
    }
}

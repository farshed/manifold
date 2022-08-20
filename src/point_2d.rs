use crate::{Point3D, Vector2D};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub};

/// A point in 2D space
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Point2D {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}

impl Point2D {
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
    pub const fn extend(&self, z: f32) -> Point3D {
        Point3D::new(self.x, self.y, z)
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        let one_t = 1. - t;
        Self::new(one_t * self.x + t * other.x, one_t * self.y + t * other.y)
    }
}

impl From<(f32, f32)> for Point2D {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Point2D { x: v.0, y: v.1 }
    }
}

impl From<Vector2D> for Point2D {
    #[inline]
    fn from(v: Vector2D) -> Self {
        Point2D { x: v.x, y: v.y }
    }
}

impl Add<Vector2D> for Point2D {
    type Output = Point2D;

    #[inline]
    fn add(self, other: Vector2D) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Vector2D> for Point2D {
    #[inline]
    fn add_assign(&mut self, other: Vector2D) {
        *self = Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Vector2D;

    #[inline]
    fn sub(self, other: Self) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Display for Point2D {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "𝛒=(")?;
        Display::fmt(&self.x, formatter)?;
        write!(formatter, ", ")?;
        Display::fmt(&self.y, formatter)?;
        write!(formatter, ")")
    }
}

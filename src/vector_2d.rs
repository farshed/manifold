use crate::Point2D;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub const ZERO: Self = Self::new(0., 0.);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a new vector with absolute values of each component
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Returns the magnitude of this vector
    #[inline]
    pub fn length(&self) -> f32 {
        self.square_length().sqrt()
    }

    /// Returns the square of this vector's magnitude
    ///
    /// This is cheaper than computing the magnitude itself
    #[inline]
    pub fn square_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        let one_t = 1. - t;
        Self::new(one_t * self.x + t * other.x, one_t * self.y + t * other.y)
    }
}

impl From<Point2D> for Vector2D {
    #[inline]
    fn from(v: Point2D) -> Self {
        Self::new(v.x, v.y)
    }
}

impl Add for Vector2D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector2D {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector2D {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vector2D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: f32) -> Self {
        self * other.recip()
    }
}

impl DivAssign<f32> for Vector2D {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.mul_assign(other.recip());
    }
}

impl Neg for Vector2D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vector2D {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "𝐯=<")?;
        Display::fmt(&self.x, formatter)?;
        write!(formatter, ", ")?;
        Display::fmt(&self.y, formatter)?;
        write!(formatter, ">")
    }
}

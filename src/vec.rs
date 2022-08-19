use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: N> Vec2 {
    pub const ZERO: Self = Self::new(0., 0.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn hypot(&self) -> f32 {
        self.x.hypot(self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl Mul<Vec2> for f32 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        other * self
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    /// Note: division by a scalar is implemented by multiplying by the reciprocal.
    ///
    /// This is more efficient but has different roundoff behavior than division.
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(&self, other: f32) -> Self {
        self * other.recip()
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.mul_assign(other.recip());
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "𝐯=(")?;
        Display::fmt(&self.x, formatter)?;
        write!(formatter, ", ")?;
        Display::fmt(&self.y, formatter)?;
        write!(formatter, ")")
    }
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

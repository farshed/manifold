mod circle;
mod color;
mod frustum;
mod line2;
mod line3;
mod line_segment2;
mod line_segment3;
mod matrix3;
mod matrix4;
mod plane;
mod point2;
mod point3;
mod polygon2;
mod polygon3;
mod quaternion;
mod rect2;
mod size2;
mod size3;
mod sphere;
mod triangle;
mod vector2;
mod vector3;
mod vertex;

pub use circle::*;
pub use color::*;
pub use frustum::*;
pub use line2::*;
pub use line3::*;
pub use matrix3::*;
pub use matrix4::*;
pub use plane::*;
pub use point2::*;
pub use point3::*;
pub use polygon2::*;
pub use polygon3::*;
pub use quaternion::*;
pub use rect2::*;
pub use size2::*;
pub use size3::*;
pub use sphere::*;
pub use triangle::*;
pub use vector2::*;
pub use vector3::*;
pub use vertex::*;

// use std::cmp::{Eq, PartialEq};
// use std::ops::{Add, Div, Mul, Neg, Sub};

// pub trait Numeric: Sized + Copy + PartialEq + Eq + Add + Sub + Mul + Div + Neg {
// const ZERO: Self = 0.into();
// const ONE: Self = 1.into();

// fn max(self, other: Self) -> Self {
//     if self > other {
//         self
//     } else {
//         other
//     }
// }

// fn min(self, other: Self) -> Self {
//     if self < other {
//         self
//     } else {
//         other
//     }
// }
// }

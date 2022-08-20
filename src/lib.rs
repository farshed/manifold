mod circle;
mod color;
mod line2;
mod line3;
mod mat;
mod point2;
mod point3;
mod polygon;
mod quad;
mod quat;
mod rect;
mod size2;
mod size3;
mod triangle;
mod vec2;
mod vec3;
mod vertex;

pub use circle::*;
pub use color::*;
pub use line2::*;
pub use line3::*;
pub use mat::*;
pub use point2::*;
pub use point3::*;
pub use quad::*;
pub use quat::*;
pub use rect::*;
pub use size2::*;
pub use size3::*;
pub use triangle::*;
pub use vec2::*;
pub use vec3::*;
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

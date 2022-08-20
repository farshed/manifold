mod circle_2d;
mod circle_3d;
mod color;
mod line_2d;
mod line_3d;
mod matrix3;
mod matrix4;
mod point_2d;
mod point_3d;
mod polygon_2d;
mod polygon_3d;
mod quaternion;
mod rect_2d;
mod size_2d;
mod size_3d;
mod triangle;
mod vector_2d;
mod vector_3d;
mod vertex;

pub use circle_2d::*;
pub use circle_3d::Circle3D as Sphere;
pub use color::*;
pub use line_2d::*;
pub use line_3d::*;
pub use matrix3::*;
pub use matrix4::*;
pub use point_2d::*;
pub use point_3d::*;
pub use polygon_2d::*;
pub use polygon_3d::*;
pub use quaternion::*;
pub use rect_2d::*;
pub use size_2d::*;
pub use size_3d::*;
pub use triangle::*;
pub use vector_2d::*;
pub use vector_3d::*;
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

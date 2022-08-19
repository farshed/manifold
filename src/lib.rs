mod circle;
mod mat;
mod point;
mod quad;
mod quat;
mod rect;
mod size;
mod triangle;
mod vec;
mod vertex;

pub use circle::*;
pub use mat::*;
pub use point::*;
pub use quad::*;
pub use quat::*;
pub use rect::*;
pub use size::*;
pub use triangle::*;
pub use vec::*;
pub use vertex::*;

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Numeric: Sized + Copy + PartialEq + Eq + Add + Sub + Mul + Div + Neg {
    const ZERO: Self = 0.into();
    const ONE: Self = 1.into();

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
}

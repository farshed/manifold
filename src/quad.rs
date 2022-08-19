use crate::{Numeric, Point2};

pub struct Quad<N: Numeric> {
    pub x: Point2<N>,
    pub y: Point2<N>,
    pub z: Point2<N>,
    pub w: Point2<N>,
}

impl<N: Numeric> Quad<N> {
    pub const fn new(x: Point2<N>, y: Point2<N>, z: Point2<N>, w: Point2<N>) -> Self {
        Self { x, y, z, w }
    }
}

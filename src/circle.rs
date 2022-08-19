use crate::{Numeric, Point2};

pub struct Circle<N: Numeric> {
    pub center: Point2<N>,
    pub radius: N,
}

impl<N: Numeric> Circle<N> {
    pub const fn new(center: Point2<N>, radius: N) -> Self {
        Self { center, radius }
    }
}

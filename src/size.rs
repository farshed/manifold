use crate::Numeric;

pub struct Size<N: Numeric> {
    pub width: N,
    pub height: N,
}

impl<N: Numeric> Size<N> {
    pub const ZERO: Self = Self::splat(N::ZERO);

    pub const fn new(width: N, height: N) -> Self {
        Self { width, height }
    }

    pub const fn splat(v: N) -> Self {
        Self::new(v, v)
    }

    pub fn is_empty(&self) -> bool {
        self.width == N::ZERO || self.height == N::ZERO
    }

    pub fn area(&self) -> N {
        self.width * self.height
    }

    pub fn aspect_ratio(self) -> N {
        self.height / self.width
    }

    pub fn scale(&self, factor: N) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }

    pub fn clamp(self, min: Size<N>, max: Size<N>) -> Self {
        let width = self.width.max(min.width).min(max.width);
        let height = self.height.max(min.height).min(max.height);
        Self::new(width, height)
    }
}

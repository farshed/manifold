pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    const ZERO: Self = Self::new(0., 0.);

    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0.0 || self.height == 0.0
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn aspect_ratio(self) -> f32 {
        self.height / self.width
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }

    pub fn clamp(self, min: Size, max: Size) -> Self {
        let width = self.width.max(min.width).min(max.width);
        let height = self.height.max(min.height).min(max.height);
        Self::new(width, height)
    }
}

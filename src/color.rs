#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn as_u32(self) -> u32 {
        self.into()
    }

    // pub fn as_f64(self) -> f64 {
    //     Self::compressed(self) as f64
    // }

    #[inline]
    fn compressed(c: Color) -> u32 {
        ((c.r as u32) << 24) + ((c.g as u32) << 16) + ((c.b as u32) << 8) + (c.a as u32) as u32
    }

    #[inline]
    fn expanded(c: u32) -> Color {
        Color {
            r: ((c >> 24) & 0xFF) as u8,
            g: ((c >> 16) & 0xFF) as u8,
            b: ((c >> 8) & 0xFF) as u8,
            a: (c & 0xFF) as u8,
        }
    }
}

impl From<u32> for Color {
    #[inline]
    fn from(color: u32) -> Self {
        Self::expanded(color)
    }
}

impl From<Color> for u32 {
    #[inline]
    fn from(color: Color) -> Self {
        Color::compressed(color)
    }
}

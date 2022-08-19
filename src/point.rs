pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ORIGIN: Point = Point::new(0., 0.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // pub fn to_vec2(&self) -> Vec2 {
    //     self.x.hypot(other)
    // }
}

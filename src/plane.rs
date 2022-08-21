use crate::Vector3;

pub struct Plane {
    pub normal: Vector3,
    pub direction: f32,
}

impl Plane {
    pub const fn new(x: f32, y: f32, z: f32, direction: f32) -> Self {
        Self {
            normal: Vector3::new(x, y, z),
            direction,
        }
    }

    pub const fn from_parts(normal: Vector3, direction: f32) -> Self {
        Self { normal, direction }
    }
}

use crate::Vector3;

/// A vector for representing 3 rotations
pub struct Quaternion {
    /// The vector part of a Quaternion
    pub vector: Vector3,
    /// The scalar part of a Qaternion that usually represents rotation
    pub scalar: f32,
}

impl Quaternion {
    /// An Identity Quaternion that represents no rotation
    pub const IDENTITY: Self = Self::from(Vector3::ZERO, 1.);

    pub const fn new(x: f32, y: f32, z: f32, rotation: f32) -> Self {
        Self {
            vector: Vector3::new(x, y, z),
            scalar: rotation,
        }
    }

    pub const fn from(vector: Vector3, rotation: f32) -> Self {
        Self {
            vector,
            scalar: rotation,
        }
    }

    /// Returns the conjugate of this quaternion
    pub fn conjugate(&self) -> Self {
        Self::new(-self.vector.x, -self.vector.y, -self.vector.z, self.scalar)
    }

    // pub fn lerp(&self, other: Quaternion, t: f32) -> Self {
    // let t_one = 1.0 - t;
    // let dot = self.vector.dot(other.vector) + self.scalar * other.scalar;
    // if dot >= 0.0 {
    //    x = Scalar.Add(Scalar.Multiply(t1, quaternion1.X), Scalar.Multiply(t, quaternion2.X));
    //    y = Scalar.Add(Scalar.Multiply(t1, quaternion1.Y), Scalar.Multiply(t, quaternion2.Y));
    //    z = Scalar.Add(Scalar.Multiply(t1, quaternion1.Z), Scalar.Multiply(t, quaternion2.Z));
    //     w= Scalar.Add(Scalar.Multiply(t1, quaternion1.W), Scalar.Multiply(t, quaternion2.W));
    // }

    // if (Scalar.GreaterThanOrEqual(dot, Scalar<T>.Zero))
    // {
    //     r.X = Scalar.Add(Scalar.Multiply(t1, quaternion1.X), Scalar.Multiply(t, quaternion2.X));
    //     r.Y = Scalar.Add(Scalar.Multiply(t1, quaternion1.Y), Scalar.Multiply(t, quaternion2.Y));
    //     r.Z = Scalar.Add(Scalar.Multiply(t1, quaternion1.Z), Scalar.Multiply(t, quaternion2.Z));
    //     r.W = Scalar.Add(Scalar.Multiply(t1, quaternion1.W), Scalar.Multiply(t, quaternion2.W));
    // }
    // else
    // {
    //     r.X = Scalar.Subtract(Scalar.Multiply(t1, quaternion1.X), Scalar.Multiply(t, quaternion2.X));
    //     r.Y = Scalar.Subtract(Scalar.Multiply(t1, quaternion1.Y), Scalar.Multiply(t, quaternion2.Y));
    //     r.Z = Scalar.Subtract(Scalar.Multiply(t1, quaternion1.Z), Scalar.Multiply(t, quaternion2.Z));
    //     r.W = Scalar.Subtract(Scalar.Multiply(t1, quaternion1.W), Scalar.Multiply(t, quaternion2.W));
    // }

    // // Normalize it.
    // T ls = Scalar.Add(Scalar.Add(Scalar.Add(Scalar.Multiply(r.X, r.X), Scalar.Multiply(r.Y, r.Y)), Scalar.Multiply(r.Z, r.Z)), Scalar.Multiply(r.W, r.W));
    // T invNorm = Scalar.Reciprocal(Scalar.Sqrt(ls));

    // r.X = Scalar.Multiply(r.X, invNorm);
    // r.Y = Scalar.Multiply(r.Y, invNorm);
    // r.Z = Scalar.Multiply(r.Z, invNorm);
    // r.W = Scalar.Multiply(r.W, invNorm);

    // return r;
    // }
}

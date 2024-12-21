use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Vec3 {
    fn dot(v1: Self, v2: Self) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }
    
    fn cross(v1: Self, v2: Self) -> Self {
        Self {
            0: v1.1 * v2.2 - v1.2 * v2.1,
            1: v1.2 * v2.0 - v1.0 * v2.2,
            2: v1.0 * v2.1 - v1.1 * v2.0,
        }
    }
}
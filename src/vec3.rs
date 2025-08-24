use std::ops;
use std::f64::consts::PI;

use crate::hittable::Facing;

use fastrand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl<T> ops::Mul<T> for Vec3
where T: Into<f64> + Copy {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self(self.0 * scalar.into(), self.1 * scalar.into(), self.2 * scalar.into())
    }
}

impl<T> ops::MulAssign<T> for Vec3
where T: Into<f64> + Copy {
    fn mul_assign(&mut self, scalar: T) {
        self.0 *= scalar.into();
        self.1 *= scalar.into();
        self.2 *= scalar.into();
    }
}

impl<T> ops::Div<T> for Vec3
where T: Into<f64> + Copy {
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Self(self.0 / scalar.into(), self.1 / scalar.into(), self.2 / scalar.into())
    }
}

impl<T> ops::DivAssign<T> for Vec3
where T: Into<f64> + Copy {
    fn div_assign(&mut self, scalar: T) {
        self.0 /= scalar.into();
        self.1 /= scalar.into();
        self.2 /= scalar.into();
    }
}

impl Vec3 {
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }
    
    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }
    
    pub fn cross(v1: Self, v2: Self) -> Self {
        Self {
            0: v1.1 * v2.2 - v1.2 * v2.1,
            1: v1.2 * v2.0 - v1.0 * v2.2,
            2: v1.0 * v2.1 - v1.1 * v2.0,
        }
    }

    pub fn length(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn random() -> Self {
        let mut rng = Rng::new();

        // theta for azimuthal, phi for polar
        let theta = rng.f64() * 2.0 * PI;
        let cos_phi = rng.f64() * 2.0 - 1.0;
        let sin_phi = (1.0 - cos_phi * cos_phi).sqrt();

        Vec3(
            sin_phi * theta.cos(),
            sin_phi * theta.sin(),
            cos_phi
        )
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * Self::dot(self, normal) * 2.0
    }

    /// requires the ray to be normalized
    pub fn refract(self, normal: Self, index_ratio: f64, facing: Facing) -> Option<Self> {
        let ray_out_perp = (self - normal * Self::dot(self, normal)) * index_ratio;

        if ray_out_perp.length_squared() > 1.0 {
            // total internal reflection
            return None;
        }

        let normal_out = match facing {
            Facing::Front => -normal,
            Facing::Back => normal
        };
        let ray_out_para = normal_out * (1.0 - ray_out_perp.length_squared()).sqrt();
        Some(ray_out_perp + ray_out_para)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Vec2(pub f64, pub f64);

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> ops::Mul<T> for Vec2
where T: Into<f64> + Copy {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self(self.0 * scalar.into(), self.1 * scalar.into())
    }
}

impl Vec2 {
    pub fn zero() -> Self {
        Self(0.0, 0.0)
    }
}

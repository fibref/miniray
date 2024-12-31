use std::ops;

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

    pub fn normalize(self) -> Self {
        let len = self.length();
        self / len
    }
}
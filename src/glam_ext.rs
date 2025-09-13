use std::f64::consts::PI;

use fastrand::Rng;
use glam::DVec3;

pub trait DVec3Ext {
    fn near_zero(self) -> bool;
    fn random() -> Self;
}

impl DVec3Ext for DVec3 {
    fn random() -> Self {
        let mut rng = Rng::new();

        // theta for azimuthal, phi for polar
        let theta = rng.f64() * 2.0 * PI;
        let cos_phi = rng.f64() * 2.0 - 1.0;
        let sin_phi = (1.0 - cos_phi * cos_phi).sqrt();

        DVec3::new(sin_phi * theta.cos(), sin_phi * theta.sin(), cos_phi)
    }

    fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

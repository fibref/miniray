use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn trace(&self) -> Vec3 {
        let t = 0.5 * (self.dir.normalize().1 + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
    }
}
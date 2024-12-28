use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn trace(&self) -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}
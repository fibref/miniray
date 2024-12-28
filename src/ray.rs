use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn trace(&self) -> Vec3 {
        let x =  self.dir.0;
        let y =  self.dir.1;
        if x * x + y * y < 0.25 {
            return Vec3(0.0, 1.0, 1.0);
        }
        Vec3(0.0, 0.0, 0.0)
    }
}
use crate::vec3::Vec3;
use crate::hittable::Hittable;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn trace(&self, obj: &dyn Hittable) -> Vec3 {
        match obj.hit(self) {
            Some(_) => Vec3(0.8, 0.0, 1.0),
            None => {
                // background
                let t = 0.5 * (self.dir.normalize().1 + 1.0);
                Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
            }
        }
    }
}
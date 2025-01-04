use crate::vec3::Vec3;
use crate::hittable::Hittable;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn trace(&self, obj: &dyn Hittable) -> Vec3 {
        match obj.hit(self) {
            Some(x) => Vec3(x.normal.0 + 1.0, x.normal.1 + 1.0, x.normal.2 + 1.0) * 0.5,
            None => {
                // background
                let a = 0.5 * (self.dir.normalize().1 + 1.0);
                Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
            }
        }
    }
}
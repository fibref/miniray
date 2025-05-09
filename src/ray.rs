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

    pub fn trace(&self, depth: u32, obj_list: &Vec<&dyn Hittable>) -> Vec3 {
        if depth <= 0 { return Vec3::zero() }

        let obj = obj_list.iter().fold(None, |acc, obj| {
            match (acc, obj.hit(self)) {
                (None, None) => None,
                (Some(x), None) => Some(x),
                (None, Some(x)) => Some(x),
                (Some(x), Some(y)) => {
                    if x.t < y.t { Some(x) } else { Some(y) }
                }
            }
        });
        match obj {
            Some(x) => {
                if let Some((scattered, attenuation)) = x.material.scatter(self, &x) {
                    scattered.trace(depth - 1, obj_list) * attenuation
                }
                else {
                    Vec3::zero()
                }
            },
            None => {
                // background
                let a = 0.5 * (self.dir.normalize().1 + 1.0);
                Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
            }
        }
    }
}
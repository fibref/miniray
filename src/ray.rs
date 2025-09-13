use crate::hittable::Hittable;

use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.dir * t
    }

    pub fn trace(&self, depth: u32, obj_list: &Vec<&dyn Hittable>, background: DVec3) -> DVec3 {
        if depth == 0 {
            return DVec3::ZERO;
        }

        let obj = obj_list.iter().fold(None, |acc, obj| {
            match (acc, obj.hit(self)) {
                // pick the closest hit
                (None, None) => None,
                (Some(x), None) => Some(x),
                (None, Some(x)) => Some(x),
                (Some(x), Some(y)) => {
                    if x.t < y.t {
                        Some(x)
                    } else {
                        Some(y)
                    }
                }
            }
        });
        match obj {
            Some(x) => {
                let emission = x.material.emit();
                let scatter = if let Some((scattered, attenuation)) = x.material.scatter(self, &x) {
                    scattered.trace(depth - 1, obj_list, background) * attenuation
                } else {
                    DVec3::ZERO
                };
                emission + scatter
            }
            None => background,
        }
    }
}

use crate::{hittable::Hittable, light::Light};

use glam::{Vec3, DVec3};

pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.dir * t
    }

    pub fn trace(&self, depth: u32, hittable: &dyn Hittable, lights: &[Box<dyn Light>], background: Vec3) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }

        let hit_info = hittable.hit(self);
        match hit_info {
            Some(x) => {
                let emission = x.material.emit();
                let scatter = if let Some((scattered, weight)) = x.material.scatter(-self.dir.normalize(), &x) {
                    scattered.trace(depth - 1, hittable, lights, background) * weight
                } else {
                    Vec3::ZERO
                };
                let lighting = lights.iter().fold(Vec3::ZERO, |acc, light| {
                    acc + light.evaluate(x.pos, x.normal, hittable) * x.material.brdf(-self.dir.normalize(), light.light_dir(x.pos), &x)
                });
                emission + scatter + lighting
            }
            None => background,
        }
    }
}

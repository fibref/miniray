use std::f32::consts::PI;

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

    pub fn trace(&self, depth: u32, obj_list: &[Box<dyn Hittable + '_>], lights: &[Box<dyn Light>], background: Vec3) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }

        let hit_info = obj_list.iter().fold(None, |acc, obj| {
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
        match hit_info {
            Some(x) => {
                let emission = x.material.emit();
                let scatter = if let Some((scattered, attenuation)) = x.material.scatter(self, &x) {
                    scattered.trace(depth - 1, obj_list, lights, background) * attenuation
                } else {
                    Vec3::ZERO
                };
                let lighting = lights.iter().fold(Vec3::ZERO, |acc, light| {
                    acc + light.evaluate(x.pos, x.normal, obj_list) / PI
                });
                emission + scatter + lighting
            }
            None => background,
        }
    }
}

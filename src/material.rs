#![allow(dead_code)]

use crate::glam_ext::DVec3Ext;
use crate::hittable::{Facing, HitRecord};
use crate::ray::Ray;
use crate::texture::Texture;

use glam::{DVec2, DVec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)>;
    fn emit(&self) -> DVec3 {
        DVec3::ZERO
    }
}

pub struct Lambertian {
    pub albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        let mut dir = normal + DVec3::random();
        // avoid zero vector
        if dir.near_zero() {
            dir = normal;
        }
        let ray_out = Ray {
            origin: hit_record.pos,
            dir,
        };
        Some((ray_out, self.albedo))
    }
}

pub struct Metal {
    pub albedo: DVec3,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        let dir = ray_in.dir.reflect(normal).normalize() + DVec3::random() * self.fuzziness;
        let ray_out = Ray {
            origin: hit_record.pos,
            dir,
        };
        if DVec3::dot(dir, normal) > 0.0 {
            Some((ray_out, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refr_index: f64,
}

impl Dielectric {
    pub fn new(refr_index: f64) -> Self {
        Self { refr_index }
    }

    pub fn reflectance_schlick(cosine: f64, refr_index: f64) -> f64 {
        let r0 = (1.0 - refr_index) / (1.0 + refr_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        let (normal_in, ri) = match hit_record.facing {
            Facing::Front => (hit_record.normal, 1.0 / self.refr_index),
            Facing::Back => (-hit_record.normal, self.refr_index),
        };

        let in_dir = ray_in.dir.normalize();
        let cosine = DVec3::dot(in_dir, hit_record.normal).abs();

        let reflectance = Dielectric::reflectance_schlick(cosine, ri);

        if fastrand::f64() > reflectance {
            let refracted = in_dir.refract(normal_in, ri);
            if refracted != DVec3::ZERO {
                return Some((
                    Ray {
                        origin: hit_record.pos,
                        dir: refracted,
                    },
                    DVec3::ONE,
                ));
            }
        }
        let refracted = in_dir.reflect(hit_record.normal);
        Some((
            Ray {
                origin: hit_record.pos,
                dir: refracted,
            },
            DVec3::ONE,
        ))
    }
}

pub struct BasicMaterial<'a> {
    albedo: &'a Texture,
}

impl<'a> BasicMaterial<'a> {
    pub fn new(albedo: &'a Texture) -> Self {
        Self { albedo }
    }
}

impl Material for BasicMaterial<'_> {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        let DVec2 { x: u, y: v } = hit_record.tex_coords;
        let albedo = self.albedo.sample(u, v);

        if albedo.near_zero() {
            return None;
        }

        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        let mut dir = normal + DVec3::random();
        // avoid zero vector
        if dir.near_zero() {
            dir = normal;
        }
        let ray_out = Ray {
            origin: hit_record.pos,
            dir,
        };
        Some((ray_out, albedo))
    }
}

pub struct Light {
    pub color: DVec3,
}

impl Light {
    pub fn new(color: DVec3) -> Self {
        Self { color }
    }
}

impl Material for Light {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, DVec3)> {
        None
    }

    fn emit(&self) -> DVec3 {
        self.color
    }
}

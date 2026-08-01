#![allow(dead_code)]

use std::f32::consts::PI as PI_f32;
use std::f64::consts::PI as PI_f64;

use crate::glam_ext::{Vec3Ext, DVec3Ext};
use crate::hittable::{Facing, HitRecord};
use crate::ray::Ray;
use crate::texture::Texture;

use fastrand::Rng;
use glam::{DVec3, Vec3};

pub trait Material {
    // view is required to be normalized
    fn scatter(&self, view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emit(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn brdf(&self, view: DVec3, light: DVec3, hit_record: &HitRecord) -> Vec3 {
        todo!()
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        let mut scattered = normal + DVec3::random();
        // avoid zero vector
        if scattered.near_zero() {
            scattered = normal;
        }
        let ray_out = Ray {
            origin: hit_record.pos,
            dir: scattered,
        };
        Some((ray_out, self.albedo))
    }

    fn brdf(&self, _view: DVec3, _light: DVec3, _hit_record: &HitRecord) -> Vec3 {
        self.albedo / PI_f32
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        let scattered = (-view).reflect(normal) + DVec3::random() * self.fuzziness;
        let ray_out = Ray {
            origin: hit_record.pos,
            dir: scattered,
        };
        if DVec3::dot(scattered, normal) > 0.0 {
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
    fn scatter(&self, view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let (normal_in, ri) = match hit_record.facing {
            Facing::Front => (hit_record.normal, 1.0 / self.refr_index),
            Facing::Back => (-hit_record.normal, self.refr_index),
        };

        let in_dir = -view;
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
                    Vec3::ONE,
                ));
            }
        }
        let refracted = in_dir.reflect(hit_record.normal);
        Some((
            Ray {
                origin: hit_record.pos,
                dir: refracted,
            },
            Vec3::ONE,
        ))
    }
}

pub struct BasicMaterial {
    albedo: Texture,
}

impl BasicMaterial {
    pub fn new(albedo: Texture) -> Self {
        Self { albedo }
    }
}

impl Material for BasicMaterial {
    fn scatter(&self, _view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let u = hit_record.tex_coords.x;
        let v = hit_record.tex_coords.y;
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

pub struct PbrMaterial {
    albedo: Texture,
    surface: Texture,
}

impl PbrMaterial {
    pub fn new(albedo: Texture, surface: Texture) -> Self {
        Self { albedo, surface }
    }

    #[allow(non_snake_case)]
    fn TrowbridgeReitzGGX(alpha: f32, normal: DVec3, half: DVec3) -> f32 {
        let ndoth = DVec3::dot(normal, half) as f32;
        if ndoth <= 0.0 {
            return 0.0;
        }
        let alpha2 = alpha * alpha;
        let denom = ndoth * ndoth * (alpha2 - 1.0) + 1.0;
        alpha2 / (PI_f32 * denom * denom)
    }

    #[allow(non_snake_case)]
    fn SmithGGX(alpha: f32, normal: DVec3, dir: DVec3) -> f32 {
        let ndotl = DVec3::dot(normal, dir) as f32;
        let alpha2 = alpha * alpha;
        let denom = ndotl + (alpha2 + (1.0 - alpha2) * ndotl * ndotl).sqrt();
        2.0 * ndotl / denom
    }

    #[allow(non_snake_case)]
    fn FresnelSchlick(albedo: Vec3, metallic: f32, half: DVec3, view: DVec3) -> Vec3 {
        let f0 = Vec3::lerp(Vec3::splat(0.04), albedo, metallic);
        let x = (1.0 - DVec3::dot(half, view) as f32).clamp(0.0, 1.0);
        f0 + (Vec3::ONE - f0) * x.powi(5)
    }
}

impl Material for PbrMaterial {
    fn scatter(&self, view: DVec3, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let u = hit_record.tex_coords.x;
        let v = hit_record.tex_coords.y;
        let albedo = self.albedo.sample(u, v);
        let surface_param = self.surface.sample(u, v);
        let roughness = surface_param[0];
        let metallic = surface_param[1];
        let alpha = roughness * roughness;
        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };

        let mut rng = Rng::new();
        let specular_approx = 0.2 + 0.8 * metallic;
        let scattered = if rng.f32_inclusive() > specular_approx {
            // Generate a diffuse ray
            normal + DVec3::random()
        } else {
            // Generate a specular ray
            let r1 = rng.f64_inclusive();
            let r2 = rng.f64_inclusive();
            
            // theta for azimuthal, phi for polar
            let theta = r1 * 2.0 * PI_f64;
            let cos_phi = ((1.0 - r2) / (r2 * (alpha * alpha - 1.0) as f64 + 1.0)).sqrt();
            let sin_phi = (1.0 - cos_phi * cos_phi).sqrt();
            DVec3::new(sin_phi * theta.cos(), sin_phi * theta.sin(), cos_phi)
        };
        let ndots = DVec3::dot(scattered, normal);
        if ndots <= 0.0 {
            return None;
        }
        
        let half = DVec3::midpoint(view, scattered).normalize();
        let d_ggx = Self::TrowbridgeReitzGGX(alpha, normal, half);
        let g_ggx = Self::SmithGGX(alpha, normal, view) * Self::SmithGGX(alpha, normal, scattered);
        let fresnel = Self::FresnelSchlick(albedo, metallic, half, view);

        let pdf_diff = ndots as f32 / PI_f32;
        let pdf_spec = d_ggx * DVec3::dot(normal, half) as f32 / (4.0 * DVec3::dot(view, half).abs() as f32);
        
        let k_diff = (Vec3::splat(1.0) - fresnel) * (1.0 - metallic);

        let brdf = k_diff * albedo / PI_f32 + d_ggx * g_ggx * fresnel / (4.0 * DVec3::dot(view, normal) as f32 * DVec3::dot(scattered, normal) as f32);
        let weight = brdf * ndots as f32 / (pdf_diff + pdf_spec);

        let ray_out = Ray {
            origin: hit_record.pos,
            dir: scattered,
        };
        Some((ray_out, weight))
    }

    // Everything are required to be normalized
    fn brdf(&self, view: DVec3, light: DVec3, hit_record: &HitRecord) -> Vec3 {
        let u = hit_record.tex_coords.x;
        let v = hit_record.tex_coords.y;
        let albedo = self.albedo.sample(u, v);
        let surface_param = self.surface.sample(u, v);
        let roughness = surface_param[0];
        let metallic = surface_param[1];
        let alpha = roughness * roughness;
        let normal = match hit_record.facing {
            Facing::Front => hit_record.normal,
            Facing::Back => -hit_record.normal,
        };
        
        let half = DVec3::midpoint(view, light).normalize();
        let d_ggx = Self::TrowbridgeReitzGGX(alpha, normal, half);
        let g_ggx = Self::SmithGGX(alpha, normal, view) * Self::SmithGGX(alpha, normal, light);
        let fresnel = Self::FresnelSchlick(albedo, metallic, half, view);

        let k_diff = (Vec3::splat(1.0) - fresnel) * (1.0 - metallic);

        k_diff * albedo / PI_f32 +
            d_ggx * g_ggx * fresnel / (4.0 * DVec3::dot(view, normal) as f32 * DVec3::dot(light, normal) as f32)
    }
}

/*
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
*/

use std::rc::Rc;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Rc<Self> {
        Rc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = if hit_record.front_face { hit_record.normal } else { -hit_record.normal };
        let mut dir = normal + Vec3::random();
        // avoid zero vector
        if dir.near_zero() {
            dir = normal;
        }
        let ray_out = Ray { origin: hit_record.pos, dir: dir };
        Some((ray_out, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Rc<Self> {
        Rc::new(Self { albedo, fuzziness })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = if hit_record.front_face { hit_record.normal } else { -hit_record.normal };
        let dir = ray_in.dir.reflect(normal).normalize() + Vec3::random() * self.fuzziness;
        let ray_out = Ray { origin: hit_record.pos, dir: dir };
        if Vec3::dot(dir, normal) > 0.0 {
            Some((ray_out, self.albedo))
        }
        else {
            None
        }
        
    }
}

pub struct Dielectric {
    refr_index: f64
}

impl Dielectric {
    pub fn new(refr_index: f64) -> Rc<Self> {
        Rc::new(Self { refr_index })
    }

    pub fn reflectance_schlick(cosine: f64, refr_index: f64) -> f64 {
        let r0 = (1.0 - refr_index) / (1.0 + refr_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let ri = if hit_record.front_face { 1.0 / self.refr_index } else { self.refr_index };

        let in_dir = ray_in.dir.normalize();
        let cosine = Vec3::dot(in_dir, hit_record.normal).abs();
        
        let reflectance = Dielectric::reflectance_schlick(cosine, ri);

        if fastrand::f64() > reflectance {
            if let Some(refracted) = in_dir.refract(hit_record.normal, ri, hit_record.front_face) {
                return Some((Ray{ origin: hit_record.pos, dir: refracted }, Vec3(1.0, 1.0, 1.0)));
            }
        }
        let refracted = in_dir.reflect(hit_record.normal);
        Some((Ray{ origin: hit_record.pos, dir: refracted }, Vec3(1.0, 1.0, 1.0)))
    }
}
use std::rc::Rc;

use crate::vec3::Vec3;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, pos: Vec3, normal: Vec3, front_face: bool) -> Option<(Ray, Vec3)>;
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
    fn scatter(&self, _ray_in: &Ray, pos: Vec3, normal: Vec3, front_face: bool) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = if front_face { normal } else { -normal };
        let mut dir = normal + Vec3::random();
        // avoid zero vector
        if dir.near_zero() {
            dir = normal;
        }
        let ray_out = Ray { origin: pos, dir: dir };
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
    fn scatter(&self, ray_in: &Ray, pos: Vec3, normal: Vec3, front_face: bool) -> Option<(Ray, Vec3)> {
        if self.albedo.near_zero() {
            return None;
        }

        let normal = if front_face { normal } else { -normal };
        let reflected = ray_in.dir.reflect(normal).normalize() + Vec3::random() * self.fuzziness;
        let ray_out = Ray { origin: pos, dir: reflected };
        if Vec3::dot(reflected, normal) > 0.0 {
            Some((ray_out, self.albedo))
        }
        else {
            None
        }
        
    }
}
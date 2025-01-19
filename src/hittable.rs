use std::rc::Rc;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub trait Hittable {
    // returns a list of hit records, from nearest to farthest
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f64,
    pub scattered: Option<(Ray, Vec3)>
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = Vec3::dot(ray.dir, oc); // h = -b / 2
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let dis_sqrt = discriminant.sqrt();
        let t1 = (h - dis_sqrt) / a;
        let t2 = (h + dis_sqrt) / a;

        if t2 <= 0.0001 {
            return None;
        }

        if t1 > 0.0001 {
            let pos = ray.at(t1);
            let normal = (pos - self.center) / self.radius;
            let front_face = true;
            Some(HitRecord {
                t: t1,
                scattered: self.material.scatter(ray, pos, normal, front_face)
            })
        }
        else {
            let pos = ray.at(t2);
            let normal = (pos - self.center) / self.radius;
            let front_face = false;
            Some(HitRecord {
                t: t2,
                scattered: self.material.scatter(ray, pos, normal, front_face)
            })
        }
    }
}
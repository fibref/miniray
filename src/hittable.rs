use crate::vec3::Vec3;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f64>;
}



pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = Vec3::dot(ray.dir, oc); // h = -b / 2
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c; 
        
        if discriminant < 0.0 {
            None
        }
        else {
            Some(1.0)
        }
    }
}
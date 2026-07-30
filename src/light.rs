use glam::{ Vec3, DVec3 };

use crate::{hittable::Hittable, ray::Ray};

pub trait Light {
    // normal must be normalized
    fn evaluate(&self, shading_point: DVec3, normal: DVec3, obj_list: &[Box<dyn Hittable + '_>]) -> Vec3;
    fn light_dir(&self, shading_point: DVec3) -> DVec3;
}

pub struct Directional {
    pub color: Vec3,
    pub dir: DVec3,
    pub intensity: f32,
}

impl Light for Directional {
    fn evaluate(&self, shading_point: DVec3, normal: DVec3, obj_list: &[Box<dyn Hittable + '_>]) -> Vec3 {
        let ray = Ray {
            origin: shading_point,
            dir: -self.dir,
        };
        let blocked = obj_list.iter().any(|obj| obj.hit(&ray).is_some());
        if blocked {
            Vec3::ZERO
        } else {
            todo!()
        }
    }
    fn light_dir(&self, _shading_point: DVec3) -> DVec3 {
        -self.dir
    }
}

pub struct Point {
    pub color: Vec3,
    pub pos: DVec3,
    pub intensity: f32,
}

impl Light for Point {
    fn evaluate(&self, shading_point: DVec3, normal: DVec3, obj_list: &[Box<dyn Hittable + '_>]) -> Vec3 {
        let ray = Ray {
            origin: shading_point,
            dir: self.pos - shading_point,
        };
        let cosine = DVec3::dot(ray.dir.normalize(), normal);
        let blocked = obj_list
            .iter()
            .any(|obj| obj.hit(&ray).is_some_and(|hit_record| hit_record.t < 1.0))
            || cosine <= 0.0;
        if blocked {
            Vec3::ZERO
        } else {
            let dist2 = ray.dir.length_squared();
            self.color * self.intensity / dist2 as f32 * cosine as f32
        }
    }
    
    fn light_dir(&self, shading_point: DVec3) -> DVec3 {
        (self.pos - shading_point).normalize()
    }
}

pub struct Spot {
    pub color: Vec3,
    pub pos: DVec3,
    pub dir: DVec3,
    pub intensity: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
}

impl Light for Spot {
    fn evaluate(&self, shading_point: DVec3, normal: DVec3 ,obj_list: &[Box<dyn Hittable + '_>]) -> Vec3 {
        todo!()
    }

    fn light_dir(&self, shading_point: DVec3) -> DVec3 {
        (self.pos - shading_point).normalize()
    }
}
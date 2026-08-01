#![allow(dead_code)]

use std::f64::consts::PI;
use std::ops;
use std::rc::Rc;

use crate::material::Material;
use crate::ray::Ray;

use glam::{DVec2, DVec3, Vec2};

pub trait Hittable {
    // Return hit information in the forward direction of the ray.
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>>;
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Front,
    Back,
}

pub struct HitRecord<'a> {
    pub t: f64,
    pub pos: DVec3,
    // Must be normalized
    pub normal: DVec3,
    pub tex_coords: DVec2,
    pub facing: Facing,
    pub material: &'a dyn Material,
}

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(pos: DVec3) -> DVec2 {
        let theta = (-pos.y).acos();
        let phi = (-pos.z).atan2(pos.x) + PI;
        DVec2::new(phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = DVec3::dot(ray.dir, oc); // h = -b / 2
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
            Some(HitRecord {
                t: t1,
                pos,
                normal,
                tex_coords: Self::get_uv(normal),
                facing: Facing::Front,
                material: self.material.as_ref(),
            })
        } else {
            let pos = ray.at(t2);
            let normal = (pos - self.center) / self.radius;
            Some(HitRecord {
                t: t2,
                pos,
                normal,
                tex_coords: Self::get_uv(normal),
                facing: Facing::Back,
                material: self.material.as_ref(),
            })
        }
    }
}

pub struct Triangle {
    vertices: [DVec3; 3],
    normal: [DVec3; 3],
    tex_coords: [Vec2; 3],
    material: Rc<dyn Material>,
}

impl Triangle {
    pub fn new(
        vertices: [DVec3; 3],
        normal: [DVec3; 3],
        tex_coords: [Vec2; 3],
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            vertices,
            normal,
            tex_coords,
            material,
        }
    }

    pub fn new_with_vertices(vertices: [DVec3; 3], material: Rc<dyn Material>) -> Self {
        let v1 = vertices[1] - vertices[0];
        let v2 = vertices[2] - vertices[0];
        Self {
            vertices,
            normal: [DVec3::cross(v1, v2).normalize(); 3],
            tex_coords: [Vec2::ZERO; 3],
            material,
        }
    }

    pub fn interpolate<T>(value: &[T; 3], (u, v): (f64, f64)) -> T
    where
        T: ops::Add<Output = T> + ops::Mul<f64, Output = T> + Copy,
    {
        value[0] * (1.0 - u - v) + value[1] * u + value[2] * v
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
        // Möller-Trumbore
        let v1 = self.vertices[1] - self.vertices[0];
        let v2 = self.vertices[2] - self.vertices[0];

        let s1 = DVec3::cross(ray.dir, v2);
        let det = DVec3::dot(v1, s1);

        // check if the ray is parallel to the triangle
        if det.abs() < 0.0001 {
            return None;
        }

        let inv_det = 1.0 / det;

        // calculate and check u
        let to_orig = ray.origin - self.vertices[0];
        let u = DVec3::dot(to_orig, s1) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        // calculate and check v
        let s2 = DVec3::cross(to_orig, v1);
        let v = DVec3::dot(ray.dir, s2) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // calculate and check t
        let t = DVec3::dot(v2, s2) * inv_det;
        if t < 0.0001 {
            return None;
        }

        let normal = Self::interpolate(&self.normal, (u, v)).normalize();
        let tex_coords = Self::interpolate(&self.tex_coords.map(|x| x.as_dvec2()), (u, v));

        Some(HitRecord {
            t,
            pos: ray.at(t),
            normal,
            tex_coords,
            facing: Facing::Front, //todo
            material: self.material.as_ref(),
        })
    }
}

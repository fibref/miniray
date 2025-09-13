#![allow(dead_code)]

use std::f64::consts::PI;
use std::ops;

use crate::material::Material;
use crate::ray::Ray;

use glam::{DVec2, DVec3};

pub trait Hittable {
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
    // normalized
    pub normal: DVec3,
    pub tex_coords: DVec2,
    pub facing: Facing,
    pub material: &'a dyn Material,
}

pub struct Sphere<'a> {
    center: DVec3,
    radius: f64,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: DVec3, radius: f64, material: &'a dyn Material) -> Self {
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

impl Hittable for Sphere<'_> {
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
                material: self.material,
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
                material: self.material,
            })
        }
    }
}

pub struct Triangle<'a> {
    vertices: [DVec3; 3],
    normal: [DVec3; 3],
    tex_coords: [DVec2; 3],
    v1: DVec3,
    v2: DVec3,
    material: &'a dyn Material,
}

impl<'a> Triangle<'a> {
    pub fn new(
        vertices: [DVec3; 3],
        normal: [DVec3; 3],
        tex_coords: [DVec2; 3],
        material: &'a dyn Material,
    ) -> Self {
        let v1 = vertices[1] - vertices[0];
        let v2 = vertices[2] - vertices[0];
        Self {
            vertices,
            normal,
            tex_coords,
            v1,
            v2,
            material,
        }
    }

    pub fn new_with_vertices(vertices: [DVec3; 3], material: &'a dyn Material) -> Self {
        let v1 = vertices[1] - vertices[0];
        let v2 = vertices[2] - vertices[0];
        Self {
            vertices,
            normal: [DVec3::cross(v1, v2).normalize(); 3],
            tex_coords: [DVec2::ZERO; 3],
            v1,
            v2,
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

impl Hittable for Triangle<'_> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
        // Möller-Trumbore

        let s1 = DVec3::cross(ray.dir, self.v2);
        let det = DVec3::dot(self.v1, s1);

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
        let s2 = DVec3::cross(to_orig, self.v1);
        let v = DVec3::dot(ray.dir, s2) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // calculate and check t
        let t = DVec3::dot(self.v2, s2) * inv_det;
        if t < 0.0001 {
            return None;
        }

        let normal = Self::interpolate(&self.normal, (u, v)).normalize();
        let tex_coords = Self::interpolate(&self.tex_coords, (u, v));

        Some(HitRecord {
            t,
            pos: ray.at(t),
            normal,
            tex_coords,
            facing: Facing::Front, //todo
            material: self.material,
        })
    }
}

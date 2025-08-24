use std::f64::consts::PI;
use std::ops;

use crate::vec3::{ Vec3, Vec2 };
use crate::ray::Ray;
use crate::material::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>>;
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Front,
    Back
}

pub struct HitRecord<'a> {
    pub t: f64,
    pub pos: Vec3,
    // normalized
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub facing: Facing,
    pub material: &'a dyn Material
}

pub struct Sphere<'a> {
    center: Vec3,
    radius: f64,
    material: &'a dyn Material
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f64, material: &'a dyn Material) -> Self {
        Self { center, radius, material }
    }

    pub fn get_uv(pos: Vec3) -> Vec2 {
        let theta = (-pos.1).acos();
        let phi = (-pos.2).atan2(pos.0) + PI;
        Vec2(phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
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
            Some(HitRecord {
                t: t1,
                pos: pos,
                normal: normal,
                tex_coords: Self::get_uv(normal),
                facing: Facing::Front,
                material: self.material
            })
        }
        else {
            let pos = ray.at(t2);
            let normal = (pos - self.center) / self.radius;
            Some(HitRecord {
                t: t2,
                pos: pos,
                normal: normal,
                tex_coords: Self::get_uv(normal),
                facing: Facing::Back,
                material: self.material
            })
        }
    }
}


pub struct Triangle<'a> {
    vertices: [Vec3; 3],
    normal: [Vec3; 3],
    tex_coords: [Vec2; 3],
    v1: Vec3,
    v2: Vec3,
    material: &'a dyn Material
}

impl<'a> Triangle<'a> {
    pub fn new(vertices: [Vec3; 3], normal: [Vec3; 3], tex_coords: [Vec2; 3], material: &'a dyn Material) -> Self {
        let v1 = vertices[1] - vertices[0];
        let v2 = vertices[2] - vertices[0];
        Self {
            vertices,
            normal,
            tex_coords,
            v1,
            v2,
            material
        }
    }

    pub fn new_with_vertices(vertices: [Vec3; 3], material: &'a dyn Material) -> Self {
        let v1 = vertices[1] - vertices[0];
        let v2 = vertices[2] - vertices[0];
        Self {
            vertices,
            normal: [Vec3::cross(v1, v2).normalize(); 3],
            tex_coords: [Vec2::zero(); 3],
            v1,
            v2,
            material
        }
    }

    pub fn interpolate<T>(value: &[T; 3], (u, v): (f64, f64)) -> T
    where T: ops::Add<Output = T> + ops::Mul<f64, Output = T> + Copy {
        value[0] * (1.0 - u - v) + value[1] * u + value[2] * v
    }
}

impl Hittable for Triangle<'_> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
        // Möller-Trumbore

        let s1 = Vec3::cross(ray.dir, self.v2);
        let det = Vec3::dot(self.v1, s1);

        // check if the ray is parallel to the triangle
        if det.abs() < 0.0001 {
            return None;
        }

        let inv_det = 1.0 / det;

        // calculate and check u
        let to_orig = ray.origin - self.vertices[0];
        let u = Vec3::dot(to_orig, s1) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        // calculate and check v
        let s2 = Vec3::cross(to_orig, self.v1);
        let v = Vec3::dot(ray.dir, s2) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // calculate and check t
        let t = Vec3::dot(self.v2, s2) * inv_det;
        if t < 0.0001 {
            return None;
        }

        let normal = Self::interpolate(&self.normal, (u, v)).normalize();
        let tex_coords = Self::interpolate(&self.tex_coords, (u, v));

        Some(HitRecord {
            t: t,
            pos: ray.at(t),
            normal: normal,
            tex_coords: tex_coords,
            facing: Facing::Front, //todo
            material: self.material
        })
    }
}
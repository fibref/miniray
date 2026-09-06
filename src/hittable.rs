#![allow(dead_code)]

use std::f64::consts::PI;
use std::ops;
use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;

use dyn_clone::DynClone;
use glam::{DVec2, DVec3, Vec2};

pub trait Hittable: Sync + DynClone {
    // Return hit information in the forward direction of the ray.
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> AABB;
}
dyn_clone::clone_trait_object!(Hittable);

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

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: DVec3,
    max: DVec3,
}

impl AABB {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let inv_dir = DVec3::ONE / ray.dir;
        let t1 = (self.min - ray.origin) * inv_dir;
        let t2 = (self.max - ray.origin) * inv_dir;

        let t_enter = DVec3::min(t1, t2);
        let t_exit = DVec3::max(t1, t2);

        let t_enter_max = t_enter.max_element();
        let t_exit_min = t_exit.min_element();

        t_exit_min >= t_enter_max && t_exit_min >= 0.0
    }

    pub fn from_multiple(aabbs: &[AABB]) -> Self {
        let mut min = DVec3::splat(f64::INFINITY);
        let mut max = DVec3::splat(f64::NEG_INFINITY);

        for aabb in aabbs {
            min = min.min(aabb.min);
            max = max.max(aabb.max);
        }

        AABB { min, max }
    }

    pub fn center(&self) -> DVec3 {
        (self.min + self.max) * 0.5
    }
}

#[derive(Clone)]
pub struct BVHNode<'a> {
    pub aabb: AABB,
    pub left: Box<dyn Hittable + 'a>,
    pub right: Box<dyn Hittable + 'a>,
}

impl<'a> BVHNode<'a> {
    pub fn build(nodes: Vec<BVHNode<'a>>) -> Self {
        if nodes.len() == 1 {
            return nodes.into_iter().next().unwrap();
        }
        let aabb = AABB::from_multiple(&nodes.iter().map(|n| n.aabb).collect::<Vec<_>>());
        let axis = fastrand::usize(0..3);

        let mut nodes_left = nodes;
        nodes_left.sort_by(|a, b| {
            let a_center = a.aabb.center();
            let b_center = b.aabb.center();
            a_center[axis]
                .partial_cmp(&b_center[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let nodes_right = nodes_left.split_off(nodes_left.len() / 2);

        Self {
            aabb,
            left: Box::new(Self::build(nodes_left)),
            right: Box::new(Self::build(nodes_right)),
        }
    }
}

impl Hittable for BVHNode<'_> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>> {
        if !self.aabb.intersect(ray) {
            return None;
        }

        let left_hit = self.left.hit(ray);
        let right_hit = self.right.hit(ray);

        // pick the closest hit
        match (left_hit, right_hit) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
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

    fn bounding_box(&self) -> AABB {
        let min = self.center - DVec3::splat(self.radius);
        let max = self.center + DVec3::splat(self.radius);
        AABB { min, max }
    }
}

#[derive(Clone)]
pub struct Triangle {
    vertices: [DVec3; 3],
    normal: [DVec3; 3],
    tex_coords: [Vec2; 3],
    material: Arc<dyn Material + Send + Sync>,
}

impl Triangle {
    pub fn new(
        vertices: [DVec3; 3],
        normal: [DVec3; 3],
        tex_coords: [Vec2; 3],
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            vertices,
            normal,
            tex_coords,
            material,
        }
    }

    pub fn new_with_vertices(vertices: [DVec3; 3], material: Arc<dyn Material + Send + Sync>) -> Self {
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

    fn bounding_box(&self) -> AABB {
        let min = self.vertices[0]
            .min(self.vertices[1])
            .min(self.vertices[2]);
        let max = self.vertices[0]
            .max(self.vertices[1])
            .max(self.vertices[2]);
        AABB { min, max }
    }
}

#[derive(Clone)]
pub struct Dummy;
impl Hittable for Dummy {
    fn hit(&self, _ray: &Ray) -> Option<HitRecord<'_>> {
        None
    }

    fn bounding_box(&self) -> AABB {
        AABB {
            min: DVec3::ZERO,
            max: DVec3::ZERO,
        }
    }
    
}

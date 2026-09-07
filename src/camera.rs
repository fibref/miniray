use std::sync::Mutex;
use std::thread::scope;

use crate::hittable::{BVHNode, Dummy, Hittable};
use crate::light::Light;
use crate::ray::Ray;
use crate::texture::Texture;

use dyn_clone::clone_box;
use fastrand::Rng;
use glam::{ Vec3, DVec3 };
use pbr::ProgressBar;

pub struct Camera {
    pub pos: DVec3,
    pub forward: DVec3,
    pub up: Option<DVec3>, // camera roll
    pub world_up: DVec3,

    pub height: u32,
    pub aspect_ratio: f64,
    pub fov: f64,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub background: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: DVec3::ZERO,
            forward: -DVec3::Z,
            up: None,
            world_up: DVec3::Y,

            height: 600,
            aspect_ratio: 4.0 / 3.0,
            fov: 90.0,
            sample_per_pixel: 1,
            max_depth: 20,
            background: Vec3::new(0.01, 0.01, 0.01),
        }
    }
}

impl Camera {
    pub fn render(&self, world: &[Box<dyn Hittable + '_>], lights: &[Box<dyn Light>]) -> Texture {
        // init
        let focal_length = self.forward.length();

        let (up, left) = match self.up {
            Some(up) => {
                let left = DVec3::cross(up, self.forward).normalize();
                (up, left)
            }
            None => {
                let left = DVec3::cross(self.world_up, self.forward).normalize();
                let up = DVec3::cross(self.forward, left).normalize();
                (up, left)
            }
        };

        let width = (self.height as f64 * self.aspect_ratio) as u32;

        let viewport_height = (self.fov / 2.0).to_radians().tan() * focal_length * 2.0;
        let viewport_width = viewport_height * width as f64 / self.height as f64;
        let pixel_size = viewport_height / self.height as f64;

        let delta_u = -left * pixel_size;
        let delta_v = -up * pixel_size;
        let viewport_upper_left = self.forward
            + (left * (viewport_width / 2.0))
            + (up * (viewport_height / 2.0))
            + delta_u / 2.0
            + delta_v / 2.0;

        let world_bvh = BVHNode::build(world.iter().map(|obj| {
            let obj = clone_box(&**obj);
            BVHNode {
                aabb: obj.bounding_box(),
                left: obj.clone(),
                right: Box::new(Dummy),
            }
        }).collect());

        let mut data: Texture = Texture::new(width, self.height);

        let mut rng = Rng::new();

        let mut pb = ProgressBar::new(self.height as u64);
        pb.show_counter = false;
        pb.show_speed = false;
        pb.message("Rendering: ");
        pb.format("[#>-]");

        let pb = Mutex::new(pb);

        scope(|s| {
            for (v, line) in data.lines_mut().enumerate() {
                let mut view_ray = Ray {
                    origin: self.pos,
                    dir: viewport_upper_left + delta_v * v as f64,
                };

                let mut rng = rng.fork();
                let world_bvh = &world_bvh;
                let pb = &pb;
                s.spawn(move || {
                    for u in 0..width {
                        let mut color = Vec3::ZERO;
                        for _ in 0..self.sample_per_pixel {
                            let offset = delta_u * (rng.f64_inclusive() - 0.5) + delta_v * (rng.f64_inclusive() - 0.5);
                            let sample_ray = Ray {
                                origin: self.pos,
                                dir: view_ray.dir + offset,
                            };
                            color += sample_ray.trace(self.max_depth, world_bvh, lights, self.background);
                        }
                        color /= self.sample_per_pixel as f32;
                        line[u as usize] = color.into();

                        view_ray.dir += delta_u;
                    }
                    pb.lock().unwrap().inc();
                });
            }
        });
        
        pb.into_inner().unwrap().finish();
        data
    }
}

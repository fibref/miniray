use crate::texture::Texture;
use crate::ray::Ray;
use crate::hittable::Hittable;

use glam::DVec3;
use fastrand::Rng;
use pbr::ProgressBar;

pub struct Camera {
    pub pos: DVec3,
    pub lookat: DVec3,
    pub up: Option<DVec3>, // camera roll
    pub world_up: DVec3,

    pub height: u32,
    pub aspect_ratio: f64,
    pub fov: f64,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub background: DVec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: DVec3::ZERO,
            lookat: -DVec3::Z,
            up: None,
            world_up: DVec3::Y,

            height: 600,
            aspect_ratio: 4.0 / 3.0,
            fov: 90.0,
            sample_per_pixel: 1,
            max_depth: 20,
            background: DVec3::new(0.01, 0.01, 0.01),
        }
    }
}

impl Camera {
    pub fn render(&self, world: &Vec<&dyn Hittable>) -> Texture {
        // init
        let view_dir = self.lookat - self.pos;
        let focal_length = view_dir.length();

        let (up, left) = match self.up {
            Some(up) => {
                let left = DVec3::cross(up, view_dir).normalize();
                (up, left)
            },
            None => {
                let left = DVec3::cross(self.world_up, view_dir).normalize();
                let up = DVec3::cross(view_dir, left).normalize();
                (up, left)
            }
        };

        let width = (self.height as f64 * self.aspect_ratio) as u32;
        
        let viewport_height = (self.fov / 2.0).to_radians().tan() * focal_length * 2.0;
        let viewport_width = viewport_height * width as f64 / self.height as f64;
        let pixel_size = viewport_height / self.height as f64;
        
        let delta_u = -left * pixel_size;
        let delta_v = -up * pixel_size;
        let viewport_upper_left = view_dir + (left * (viewport_width / 2.0)) + (up * (viewport_height / 2.0)) + delta_u / 2.0 + delta_v / 2.0;

        // todo
        let mut data: Texture = Texture::new(1066, 600);

        let mut rng = Rng::new();
        let mut offsets: Vec<DVec3> = Vec::with_capacity(self.sample_per_pixel as usize);
        for _ in 0..self.sample_per_pixel {
            let offset = delta_u * (rng.f64() - 0.5) + delta_v * (rng.f64() - 0.5);
            offsets.push(offset);
        }

        let mut pb = ProgressBar::new(self.height as u64);
        pb.show_counter = false;
        pb.show_speed = false;
        pb.message("Rendering: ");
        pb.format("[#>-]");

        let mut view_ray = Ray {
            origin: self.pos,
            dir: viewport_upper_left
        };
        for v in 0..self.height {
            view_ray.dir = viewport_upper_left + delta_v * v as f64;

            for u in 0..width {
                let color = offsets.iter().fold(DVec3::ZERO, |acc, offset| {
                    let sample_ray = Ray { origin: self.pos, dir: view_ray.dir + *offset };
                    acc + sample_ray.trace(self.max_depth, world, self.background)
                }) / self.sample_per_pixel as f64;
                data.set(u, v, color);

                view_ray.dir += delta_u;
            }
            pb.inc();
        }
        pb.finish();
        data
    }
}
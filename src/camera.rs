use crate::vec3::Vec3;
use crate::texture::Texture;
use crate::ray::Ray;
use crate::hittable::Hittable;

pub struct Camera {
    pub pos: Vec3,
    pub lookat: Vec3,
    pub world_up: Vec3,

    pub width: u32,
    pub height: u32,
    pub fov: f64
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vec3(0.0, 0.0, 0.0),
            lookat: Vec3(0.0, 0.0, -1.0),
            world_up: Vec3(0.0, 1.0, 0.0),

            width: 800,
            height: 600,
            fov: 90.0
        }
    }
}

impl Camera {
    pub fn render(&self, world: &Vec<&dyn Hittable>) -> Texture {
        // init
        let view_dir = self.lookat - self.pos;
        let focal_length = view_dir.length();

        let left = Vec3::cross(self.world_up, view_dir).normalize();
        let up = Vec3::cross(view_dir, left).normalize();
        
        let viewport_height = (self.fov / 2.0).to_radians().tan() * focal_length * 2.0;
        let viewport_width = viewport_height * self.width as f64 / self.height as f64;
        let pixel_size = viewport_height / self.height as f64;
        
        let delta_u = -left * pixel_size;
        let delta_v = -up * pixel_size;
        let viewport_upper_left = view_dir + (left * (viewport_width / 2.0)) + (up * (viewport_height / 2.0)) + delta_u / 2.0 + delta_v / 2.0;

        let mut data: Texture = Texture::new(self.width, self.height);

        let mut view_ray = Ray {
            origin: self.pos,
            dir: viewport_upper_left
        };
        for v in 0..self.height {
            view_ray.dir = viewport_upper_left + delta_v * v;
            for u in 0..self.width {
                data.set(u, v, view_ray.trace(&world));
                view_ray.dir += delta_u;
            }
        }
        data
    }
}
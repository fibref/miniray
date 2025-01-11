use crate::vec3::Vec3;
use crate::texture::Texture;
use crate::ray::Ray;
use crate::hittable::Hittable;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub focal_length: f64
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            width: 800,
            height: 600,
            fov: 90.0,
            focal_length: 1.0
        }
    }
}

impl Camera {
    pub fn render(&self, world: &Vec<&dyn Hittable>) -> Texture {
        let viewport_height = (self.fov / 2 as f64).to_radians().tan() * self.focal_length * 2.0;
        let viewport_width = viewport_height * self.width as f64 / self.height as f64;
        let pixel_size = viewport_height / self.height as f64;
        let viewport_upper_left = Vec3(-viewport_width / 2.0 + pixel_size / 2.0, viewport_height / 2.0 - pixel_size / 2.0, -self.focal_length);

        let mut data: Texture = Texture::new(self.width, self.height);

        let mut view_ray = Ray {
            origin: Vec3(0.0, 0.0, 0.0),
            dir: viewport_upper_left
        };
        for v in 0..self.height {
            view_ray.dir = viewport_upper_left + Vec3(0.0, -pixel_size * v as f64, 0.0);
            for u in 0..self.width {
                data.set(u, v, view_ray.trace(&world));
                view_ray.dir += Vec3(pixel_size, 0.0, 0.0);
            }
        }
        data
    }
}
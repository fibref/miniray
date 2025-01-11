use std::fs::File;
use vec3::Vec3;
use texture::Texture;
use ray::Ray;
use hittable::{ Hittable, Sphere };

mod vec3;
mod texture;
mod ray;
mod hittable;

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 600;
const FOV: f64 = 90.0;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    println!("Hello, world!");

    // image setup
    let out_file = File::create("output.png").expect("Unable to create file");
    let mut encoder = png::Encoder::new(out_file, IMG_WIDTH, IMG_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().expect("Unable to write header");
    
    let mut data: Texture = Texture::new(IMG_WIDTH, IMG_HEIGHT);

    let viewport_height = (FOV / 2 as f64).to_radians().tan() * FOCAL_LENGTH * 2.0;
    let viewport_width = viewport_height * IMG_WIDTH as f64 / IMG_HEIGHT as f64;
    let pixel_size = viewport_height / IMG_HEIGHT as f64;
    let viewport_upper_left = Vec3(-viewport_width / 2.0 + pixel_size / 2.0, viewport_height / 2.0 - pixel_size / 2.0, -FOCAL_LENGTH);

    // world
    let mut world: Vec<&dyn Hittable> = Vec::new();
    world.push(&Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    });
    world.push(&Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    });

    let mut view_ray = Ray {
        origin: Vec3(0.0, 0.0, 0.0),
        dir: viewport_upper_left
    };
    for v in 0..IMG_HEIGHT {
        view_ray.dir = viewport_upper_left + Vec3(0.0, -pixel_size * v as f64, 0.0);
        for u in 0..IMG_WIDTH {
            data.set(u, v, view_ray.trace(&world));
            view_ray.dir += Vec3(pixel_size, 0.0, 0.0);
        }
    }
    
    writer.write_image_data(data.rgb_buffer().as_slice()).expect("Unable to write image data");
}

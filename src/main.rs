use std::fs::File;

use vec3::Vec3;
use camera::Camera;
use hittable::{ Hittable, Sphere };
use material::{ Lambertian, Metal };

mod vec3;
mod texture;
mod ray;
mod hittable;
mod camera;
mod material;

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 600;

fn main() {
    println!("Hello, world!");

    // image setup
    let out_file = File::create("output.png").expect("Unable to create file");
    let mut encoder = png::Encoder::new(out_file, IMG_WIDTH, IMG_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().expect("Unable to write header");

    // world
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let mat_center = Lambertian::new(Vec3(0.1, 0.2, 0.5));
    let mat_left = Metal::new(Vec3(0.7, 0.7, 0.7), 0.2);
    let mat_right = Metal::new(Vec3(0.1, 0.1, 0.11), 0.7);
    let mat_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));

    let center = Sphere {
        center: Vec3(0.0, 0.0, -1.1),
        radius: 0.5,
        material: mat_center.clone()
    };
    let left = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: mat_left.clone()
    };
    let right = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: mat_right.clone()
    };
    let ground = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: mat_ground.clone()
    };
    world.push(&center);
    world.push(&left);
    world.push(&right);
    world.push(&ground);

    // camera
    let cam = Camera {
        width: IMG_WIDTH,
        height: IMG_HEIGHT,
        sample_per_pixel: 10,
        max_depth: 30,
        ..Default::default()
    };
    let data = cam.render(&world);
    
    writer.write_image_data(data.rgb_buffer().as_slice()).expect("Unable to write image data");
}

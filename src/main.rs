use std::fs::File;

use vec3::Vec3;
use camera::Camera;
use hittable::{ Hittable, Sphere, Triangle };
use material::{ Lambertian, Metal, Dielectric };

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
    let mat_left = Dielectric::new(1.5);
    let mat_bubble = Dielectric::new(1.0 / 1.5);
    let mat_right = Metal::new(Vec3(0.1, 0.1, 0.11), 0.7);
    let mat_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));

    /* let center = Sphere::new(
        Vec3(0.0, 0.0, -1.2),
        0.5,
        mat_center.clone()
    ); */
    let left = Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        mat_left.clone()
    );
    let bubble = Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble.clone()
    );
    let right = Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        mat_right.clone()
    );
    let ground = Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        mat_ground.clone()
    );
    let tri = Triangle::new(
        [Vec3(-2.0, 1.0, -4.0), Vec3(2.0, 0.0, -4.0), Vec3(0.0, 2.0, -5.0)],
        mat_center
    );
    // world.push(&center);
    world.push(&left);
    world.push(&bubble);
    world.push(&right);
    world.push(&ground);
    world.push(&tri);

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

use std::fs::File;
use vec3::Vec3;
use camera::Camera;
use hittable::{ Hittable, Sphere };

mod vec3;
mod texture;
mod ray;
mod hittable;
mod camera;

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
    world.push(&Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    });
    world.push(&Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    });

    // camera
    let cam = Camera {
        width: IMG_WIDTH,
        height: IMG_HEIGHT,
        sample_per_pixel: 10,
        ..Default::default()
    };
    let data = cam.render(&world);
    
    writer.write_image_data(data.rgb_buffer().as_slice()).expect("Unable to write image data");
}

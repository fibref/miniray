use vec3::Vec3;
use texture::Texture;
use camera::Camera;
use hittable::{ Hittable, Sphere, Triangle };
use material::{ Lambertian, Metal, Dielectric, BasicMaterial, Light };

use image::ImageReader;

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

    let earthmap_img = ImageReader::open("earthmap.jpg").unwrap().decode().unwrap().to_rgb8();
    let (earthmap_width, earthmap_height) = earthmap_img.dimensions();
    let earthmap_buf = earthmap_img.as_raw();
    let earthmap = Texture::from_rgb_buffer(earthmap_width, earthmap_height, earthmap_buf);

    // world
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let mat_center = BasicMaterial::new(&earthmap);
    let mat_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let mat_light = Light::new(Vec3(4.0, 4.0, 4.0));

    let center = Sphere::new(
        Vec3(0.0, 0.0, -1.2),
        0.5,
        &mat_center
    );
    let ground = Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &mat_ground
    );
    let light_1 = Triangle::new(
        [Vec3(1.8, -0.2, -0.5), Vec3(1.8, 0.8, -0.5), Vec3(1.2, -0.2, -1.5)],
        &mat_light
    );
    let light_2 = Triangle::new(
        [Vec3(1.2, 0.8, -1.5), Vec3(1.8, 0.8, -0.5), Vec3(1.2, -0.2, -1.5)],
        &mat_light
    );
    
    world.push(&center);
    world.push(&ground);
    world.push(&light_1);
    world.push(&light_2);

    // camera
    let cam = Camera {
        width: IMG_WIDTH,
        height: IMG_HEIGHT,
        sample_per_pixel: 400,
        max_depth: 30,
        ..Default::default()
    };
    let data = cam.render(&world);
    
    image::save_buffer("output.png", data.rgb_buffer().as_slice(), IMG_WIDTH, IMG_HEIGHT, image::ColorType::Rgb8).expect("Unable to write image data");
}

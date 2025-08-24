use vec3::{ Vec3, Vec2 };
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

    let awesomeface_img = ImageReader::open("awesomeface.png").unwrap().decode().unwrap().to_rgb8();
    let (awesomeface_width, awesomeface_height) = awesomeface_img.dimensions();
    let awesomeface_buf = awesomeface_img.as_raw();
    let awesomeface = Texture::from_rgb_buffer(awesomeface_width, awesomeface_height, awesomeface_buf);

    // world
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let mat_center = BasicMaterial::new(&earthmap);
    let mat_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let mat_light = Light::new(Vec3(4.0, 4.0, 4.0));
    let mat_tri = BasicMaterial::new(&awesomeface);

    let center = Sphere::new(
        Vec3(-0.6, 0.0, -1.2),
        0.5,
        &mat_center
    );
    let ground = Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &mat_ground
    );
    let light_1 = Triangle::new_with_vertices(
        [Vec3(1.8, -0.2, -0.5), Vec3(1.8, 0.8, -0.5), Vec3(1.2, -0.2, -1.5)],
        &mat_light
    );
    let light_2 = Triangle::new_with_vertices(
        [Vec3(1.2, 0.8, -1.5), Vec3(1.8, 0.8, -0.5), Vec3(1.2, -0.2, -1.5)],
        &mat_light
    );
    let tri_1 = Triangle::new(
        [Vec3(0.5, -0.3, -1.0), Vec3(0.5, -0.3, -0.5), Vec3(-0.2, -0.3, -1.0)],
        [Vec3(0.0, 1.0, 0.0); 3],
        [Vec2(1.0, 1.0), Vec2(1.0, 0.0), Vec2(0.0, 1.0)],
        &mat_tri
    );
    let tri_2 = Triangle::new(
        [Vec3(-0.2, -0.3, -0.5), Vec3(0.5, -0.3, -0.5), Vec3(-0.2, -0.3, -1.0)],
        [Vec3(0.0, 1.0, 0.0); 3],
        [Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(0.0, 1.0)],
        &mat_tri
    );
    world.push(&center);
    world.push(&ground);
    world.push(&light_1);
    world.push(&light_2);
    world.push(&tri_1);
    world.push(&tri_2);

    // camera
    let cam = Camera {
        width: IMG_WIDTH,
        height: IMG_HEIGHT,
        sample_per_pixel: 400,
        max_depth: 30,
        background: Vec3(0.03, 0.03, 0.03),
        ..Default::default()
    };
    let data = cam.render(&world);
    
    image::save_buffer("output.png", data.rgb_buffer().as_slice(), IMG_WIDTH, IMG_HEIGHT, image::ColorType::Rgb8).expect("Unable to write image data");
}

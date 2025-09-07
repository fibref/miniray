use texture::Texture;
use camera::Camera;
use scene::Scene;
use hittable::{ Hittable, Sphere, Triangle };
use material::{ Lambertian, Metal, Dielectric, BasicMaterial, Light };

use glam::{ DVec2, DVec3 };
use image::ImageReader;

mod glam_ext;
mod texture;
mod ray;
mod hittable;
mod camera;
mod material;
mod scene;

fn main() {
    println!("Hello, world!");

    let mut scene = Scene::import("blender-test.gltf").into_iter().next().unwrap();

    let mat_light = Light::new(DVec3::new(4.0, 4.0, 4.0));
    let light_1 = Triangle::new_with_vertices(
        [DVec3::new(2.2, 1.5, -0.5), DVec3::new(2.2, 2.3, -0.5), DVec3::new(1.2, 1.5, -1.5)],
        &mat_light
    );
    let light_2 = Triangle::new_with_vertices(
        [DVec3::new(1.2, 2.3, -1.5), DVec3::new(2.2, 2.3, -0.5), DVec3::new(1.2, 1.5, -1.5)],
        &mat_light
    );

    scene.camera.sample_per_pixel = 400;
    
    let mut list = scene.ref_vec();
    list.push(&light_1);
    list.push(&light_2);

    let data = scene.camera.render(&list);
    
    // todo
    image::save_buffer("output.png", data.rgb_buffer().as_slice(), 1066, 600, image::ColorType::Rgb8).expect("Unable to write image data");
}

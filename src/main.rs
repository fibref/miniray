use std::time::Instant;

use scene::Scene;


mod camera;
mod glam_ext;
mod hittable;
mod light;
mod material;
mod ray;
mod scene;
mod texture;

fn main() {
    println!("Hello, world!");

    let mut scene = Scene::import("two_boxes.gltf")
        .into_iter()
        .next()
        .unwrap();

    scene.camera.sample_per_pixel = 30;

    let start = Instant::now();
    let data = scene.camera.render(&scene.hittables, &scene.lights);
    let duration = start.elapsed();
    println!("\nRendered in {:.3}s", duration.as_secs_f32());

    image::save_buffer(
        "output.png",
        data.rgb_buffer().as_slice(),
        data.width,
        data.height,
        image::ColorType::Rgb8,
    )
    .expect("Unable to write image data");
}

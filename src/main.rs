use std::fs::File;
use texture::Texture;

mod vec3;
mod texture;

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 600;

fn main() {
    println!("Hello, world!");

    let out_file = File::create("output.png").expect("Unable to create file");
    let mut encoder = png::Encoder::new(out_file, IMG_WIDTH, IMG_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().expect("Unable to write header");
    
    let data: Texture = Texture::new(IMG_WIDTH, IMG_HEIGHT);
    
    writer.write_image_data(data.rgb_buffer().as_slice()).expect("Unable to write image data");
}

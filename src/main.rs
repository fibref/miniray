use std::fs::File;
use texture::Texture;

mod vec3;
mod texture;

fn main() {
    println!("Hello, world!");

    let out_file = File::create("output.png").expect("Unable to create file");
    let mut encoder = png::Encoder::new(out_file, 800, 600);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().expect("Unable to write header");
    
    let data: Texture<800, 600> = Texture::new();
    
    writer.write_image_data(data.rgb_buffer().as_slice()).expect("Unable to write image data");
}

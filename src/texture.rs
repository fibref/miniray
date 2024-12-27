use crate::vec3::Vec3;

pub struct Texture {
    width: u32,
    height: u32,
    buffer: Vec<Vec3>
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            buffer: vec![Vec3(0.0, 0.0, 0.0); (width * height) as usize]
        }
    }

    pub fn rgb_buffer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity((self.width * self.height * 3) as usize);
        for color in &self.buffer {
            buf.push((color.0 * 255.0) as u8);
            buf.push((color.1 * 255.0) as u8);
            buf.push((color.2 * 255.0) as u8);
        }
        buf
    }
}
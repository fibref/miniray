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
            buffer: vec![Vec3::zero(); (width * height) as usize]
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Vec3) {
        self.buffer[(y * self.width + x) as usize] = color;
    }

    pub fn rgb_buffer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity((self.width * self.height * 3) as usize);
        for color in &self.buffer {
            let color_gamma = Self::to_gamma(*color);
            buf.push((color_gamma.0 * 255.0) as u8);
            buf.push((color_gamma.1 * 255.0) as u8);
            buf.push((color_gamma.2 * 255.0) as u8);
        }
        buf
    }

    fn to_gamma(color: Vec3) -> Vec3 {
        Vec3(
            color.0.clamp(0.0, 1.0).sqrt(),
            color.1.clamp(0.0, 1.0).sqrt(),
            color.2.clamp(0.0, 1.0).sqrt(),
        )
    }
}
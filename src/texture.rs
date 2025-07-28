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

    pub fn from_rgb_buffer(width: u32, height: u32, buffer: &Vec<u8>) -> Self {
        let mut buf: Vec<Vec3> = Vec::with_capacity((width * height) as usize);
        for i in buffer.chunks_exact(3) {
            buf.push(
                Self::to_linear(Vec3(i[0] as f64 / 255.0, i[1] as f64 / 255.0, i[2] as f64 / 255.0))
            )
        }
        Self {
            width: width,
            height: height,
            buffer: buf
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Vec3) {
        self.buffer[(y * self.width + x) as usize] = color;
    }

    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip v to image space
        let x = (u * self.width as f64) as u32;
        let y = (v * self.height as f64) as u32;
        self.buffer[(y * self.width + x) as usize]
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

    fn to_linear(color: Vec3) -> Vec3 {
        Vec3(
            color.0.powi(2),
            color.1.powi(2),
            color.2.powi(2),
        )
    }
}
#![allow(dead_code)]

use glam::DVec3;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    buffer: Vec<DVec3>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![DVec3::ZERO; (width * height) as usize],
        }
    }

    pub fn from_rgb_buffer(width: u32, height: u32, buffer: &[u8]) -> Self {
        let mut buf: Vec<DVec3> = Vec::with_capacity((width * height) as usize);
        for i in buffer.chunks_exact(3) {
            buf.push(Self::to_linear(DVec3::new(
                i[0] as f64 / 255.0,
                i[1] as f64 / 255.0,
                i[2] as f64 / 255.0,
            )))
        }
        Self {
            width,
            height,
            buffer: buf,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: DVec3) {
        self.buffer[(y * self.width + x) as usize] = color;
    }

    pub fn sample(&self, u: f64, v: f64) -> DVec3 {
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
            buf.push((color_gamma.x * 255.0) as u8);
            buf.push((color_gamma.y * 255.0) as u8);
            buf.push((color_gamma.z * 255.0) as u8);
        }
        buf
    }

    fn to_gamma(color: DVec3) -> DVec3 {
        DVec3::new(
            color.x.clamp(0.0, 1.0).sqrt(),
            color.y.clamp(0.0, 1.0).sqrt(),
            color.z.clamp(0.0, 1.0).sqrt(),
        )
    }

    fn to_linear(color: DVec3) -> DVec3 {
        DVec3::new(color.x.powi(2), color.y.powi(2), color.z.powi(2))
    }
}

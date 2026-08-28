#![allow(dead_code)]

use glam::{ Vec3, Vec3A };

pub struct Texture {
    pub width: u32,
    pub height: u32,
    buffer: Vec<Vec3A>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![Vec3A::ZERO; (width * height) as usize],
        }
    }

    pub fn plain(color: Vec3) -> Self {
        Self {
            width: 1,
            height: 1,
            buffer: vec![color.into(); 1],
        }
    }

    pub fn from_linear(width: u32, height: u32, buffer: &[u8]) -> Self {
        let mut buf: Vec<Vec3A> = Vec::with_capacity((width * height) as usize);
        for i in buffer.chunks_exact(3) {
            buf.push(Vec3A::new(
                i[0] as f32 / 255.0,
                i[1] as f32 / 255.0,
                i[2] as f32 / 255.0,
            ))
        }
        Self {
            width,
            height,
            buffer: buf,
        }
    }

    pub fn from_srgb(width: u32, height: u32, buffer: &[u8]) -> Self {
        let mut buf: Vec<Vec3A> = Vec::with_capacity((width * height) as usize);
        for i in buffer.chunks_exact(3) {
            buf.push(Self::to_linear(Vec3A::new(
                i[0] as f32 / 255.0,
                i[1] as f32 / 255.0,
                i[2] as f32 / 255.0,
            )))
        }
        Self {
            width,
            height,
            buffer: buf,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Vec3) {
        self.buffer[(y * self.width + x) as usize] = color.into();
    }

    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip v to image space
        let x = ((u * self.width as f64) as u32).clamp(0, self.width - 1);
        let y = ((v * self.height as f64) as u32).clamp(0, self.height - 1);
        self.buffer[(y * self.width + x) as usize].into()
    }

    pub fn rgb_buffer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity((self.width * self.height * 3) as usize);
        for color in &self.buffer {
            let color_gamma = Self::to_gamma(Self::tone_mapping(*color * (1.0)));
            buf.push((color_gamma.x * 255.0) as u8);
            buf.push((color_gamma.y * 255.0) as u8);
            buf.push((color_gamma.z * 255.0) as u8);
        }
        buf
    }

    fn to_gamma(color: Vec3A) -> Vec3A {
        Vec3A::new(
            color.x.clamp(0.0, 1.0).sqrt(),
            color.y.clamp(0.0, 1.0).sqrt(),
            color.z.clamp(0.0, 1.0).sqrt(),
        )
    }

    fn tone_mapping(color: Vec3A) -> Vec3A {
        color / (color + Vec3A::new(1.0, 1.0, 1.0))
    }

    fn to_linear(color: Vec3A) -> Vec3A {
        Vec3A::new(color.x.powi(2), color.y.powi(2), color.z.powi(2))
    }
}

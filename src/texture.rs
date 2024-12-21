use crate::vec3;

pub struct Texture<const W: usize, const H: usize> {
    buffer: Vec<vec3::Vec3>
}

impl<const W: usize, const H: usize> Texture<W, H> {
    pub fn new() -> Self {
        Self { buffer: vec![vec3::Vec3(0.0, 0.0, 0.0); W * H] }
    }

    pub fn rgb_buffer(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(W * H * 3);
        for color in &self.buffer {
            buf.push((color.0 * 255.0) as u8);
            buf.push((color.1 * 255.0) as u8);
            buf.push((color.2 * 255.0) as u8);
        }
        buf
    }
}
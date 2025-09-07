use crate::hittable::{Hittable, Triangle};
use crate::camera::Camera;
use crate::material::Lambertian;

use gltf::mesh::Reader;
use gltf::Buffer;
use gltf::{buffer::Data, Mesh, Node};
use glam::{ DMat4, Mat4, Vec3, DVec3 };

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub camera: Camera,
}

impl Scene {
    pub fn import(file_path: &str) -> Vec<Self> {
        let (document, buffers, _) = gltf::import(file_path).unwrap();
        
        document.scenes().map(|scene| {
            let mut result = Scene {
                hittables: Vec::new(),
                camera: Camera::default(),
            };

            for node in scene.nodes() {
                result.process_node(&node, DMat4::IDENTITY, &buffers);
            }
            result
        }).collect()
    }

    pub fn ref_vec(&self) -> Vec<&dyn Hittable> {
        self.hittables.iter().map(|h| h.as_ref()).collect()
    }

    fn process_node(&mut self, node: &Node, parent_transform: DMat4, buffers: &Vec<Data>) {
        let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix()).as_dmat4();
        let transform = parent_transform * local_transform;

        // todo
        if let Some(camera) = Self::get_camera(node, transform) {
            self.camera = camera;
        }

        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                self.build_triangles(&reader, transform);
            }
        }
    }
    
    fn get_camera(node: &Node, transform: DMat4) -> Option<Camera> {
        let camera = node.camera()?;
        let (aspect_ratio, fov) = match camera.projection() {
            gltf::camera::Projection::Perspective(perspective) => {
                (perspective.aspect_ratio().unwrap() as f64, perspective.yfov().to_degrees() as f64)
            },
            gltf::camera::Projection::Orthographic(_) => todo!()
        };

        let pos = transform.transform_point3(DVec3::ZERO);
        let lookat = transform.transform_point3(-DVec3::Z).normalize();
        let up = transform.transform_vector3(DVec3::Y).normalize();

        Some(Camera {
            pos,
            lookat,
            up: Some(up),
            aspect_ratio,
            fov,
            ..Default::default()
        })
    }

    fn build_triangles<'a, 's, F>(&mut self, reader: &Reader<'a, 's, F>, transform: DMat4)
    where F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]> {
        let positions = match reader.read_positions() {
            Some(positions) => positions.map(|p| {
                let pos_vec4 = transform * Vec3::from_array(p).extend(1.0).as_dvec4();
                pos_vec4.truncate()
            }).collect::<Vec<_>>(),
            None => todo!(),
        };

        // try using indices
        if let Some(indices) = reader.read_indices() {
            let indices: Vec<u32> = indices.into_u32().collect();
            // assume primitive type is triangles
            for idx in indices.chunks_exact(3) {
                let vertices = [
                    positions[idx[0] as usize],
                    positions[idx[1] as usize],
                    positions[idx[2] as usize],
                ];
                let triangle = Triangle::new_with_vertices(vertices, &_material); //todo
                self.hittables.push(Box::new(triangle));
            }
        }
        else {
            todo!();
        }
    }
}

// todo
static _material: Lambertian = Lambertian {
    albedo: DVec3::new(0.8, 0.8, 0.8),
};

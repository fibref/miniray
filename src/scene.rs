use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::{Hittable, Triangle};
use crate::light::{self, Light};
use crate::material::{Material, PbrMaterial};
use crate::texture::Texture;

use glam::{DMat4, DVec3, Mat4, Vec2, Vec3};
use gltf::mesh::Mode::Triangles;
use gltf::{Buffer, Primitive};
use gltf::khr_lights_punctual::Kind::{Directional, Point, Spot};
use gltf::mesh::Reader;
use gltf::{Node, buffer::Data};

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
    materials: Vec<Rc<dyn Material>>,
}

impl Scene {
    pub fn import(file_path: &str) -> Vec<Self> {
        let (document, buffers, _) = gltf::import(file_path).unwrap();

        document
            .scenes()
            .map(|scene| {
                let mut result = Scene {
                    hittables: Vec::new(),
                    lights: Vec::new(),
                    camera: Camera::default(),
                    materials: Vec::new(),
                };

                for node in scene.nodes() {
                    result.process_node(&node, DMat4::IDENTITY, &buffers);
                }
                result
            })
            .collect()
    }

    fn process_node(&mut self, node: &Node, parent_transform: DMat4, buffers: &[Data]) {
        let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix()).as_dmat4();
        let transform = parent_transform * local_transform;

        // todo
        if let Some(camera) = Self::get_camera(node, transform) {
            self.camera = camera;
        }
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                self.build_triangles(&primitive, &reader, transform);
            }
        }
        if let Some(light) = Self::get_light(node, transform) {
            self.lights.push(light);
        }
    }

    fn get_camera(node: &Node, transform: DMat4) -> Option<Camera> {
        let camera = node.camera()?;
        let (aspect_ratio, fov) = match camera.projection() {
            gltf::camera::Projection::Perspective(perspective) => (
                perspective.aspect_ratio().unwrap() as f64,
                perspective.yfov().to_degrees() as f64,
            ),
            gltf::camera::Projection::Orthographic(_) => todo!(),
        };

        let pos = transform.transform_point3(DVec3::ZERO);
        let forward = transform.transform_vector3(DVec3::NEG_Z).normalize();
        let up = transform.transform_vector3(DVec3::Y).normalize();

        Some(Camera {
            pos,
            forward,
            up: Some(up),
            aspect_ratio,
            fov,
            ..Default::default()
        })
    }

    fn build_triangles<'a, 's, F>(&mut self, primitive: &Primitive, reader: &Reader<'a, 's, F>, transform: DMat4)
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        if primitive.mode() != Triangles {
            todo!()
        }
        let mut uv_set = 0;
        let pbr_mat = primitive.material().pbr_metallic_roughness();
        let metallic_factor = pbr_mat.metallic_factor();
        let roughness_factor = pbr_mat.roughness_factor();

        if let Some(tex_info) = pbr_mat.metallic_roughness_texture() {
            uv_set = tex_info.tex_coord();
            todo!();
        }
        let albedo_tex = Texture::plain(Vec3::splat(0.8)); // todo
        let surface_tex = Texture::plain(Vec3::new(roughness_factor, metallic_factor, 0.0));
        let material = Rc::new(PbrMaterial::new(albedo_tex, surface_tex));
        self.materials.push(material.clone());
        
        let positions = reader.read_positions()
            .unwrap()
            .map(|p| {
                let pos_vec4 = transform * Vec3::from_array(p).extend(1.0).as_dvec4();
                pos_vec4.truncate()
            })
            .collect::<Vec<_>>();
        let tex_coords: Vec<Vec2> = reader.read_tex_coords(uv_set)
            .unwrap()
            .into_f32()
            .map(|p| Vec2::new(p[0], p[1]))
            .collect();

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
                let v1 = vertices[1] - vertices[0];
                let v2 = vertices[2] - vertices[0];
                let normal = [DVec3::cross(v1, v2).normalize(); 3];
                let uvs = [
                    tex_coords[idx[0] as usize],
                    tex_coords[idx[1] as usize],
                    tex_coords[idx[2] as usize],
                ];
                let triangle = Triangle::new(vertices, normal, uvs, material.clone());
                self.hittables.push(Box::new(triangle));
            }
        }
        // no indices
        else {
            todo!();
        }
    }

    fn get_light(node: &Node, transform: DMat4) -> Option<Box<dyn Light>> {
        let light = node.light()?;
        match light.kind() {
            Directional => Some(Box::new(light::Directional {
                color: light.color().into(),
                dir: transform.transform_vector3(DVec3::NEG_Z),
                intensity: light.intensity(),
            })),
            Point => Some(Box::new(light::Point {
                color: light.color().into(),
                pos: transform.transform_point3(DVec3::ZERO),
                intensity: light.intensity(),
            })),
            Spot { inner_cone_angle, outer_cone_angle } => Some(Box::new(light::Spot {
                color: light.color().into(),
                pos: transform.transform_point3(DVec3::ZERO),
                dir: transform.transform_vector3(DVec3::NEG_Z),
                intensity: light.intensity(),
                inner_angle: inner_cone_angle,
                outer_angle: outer_cone_angle,
            })),
        }
    }
}

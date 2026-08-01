use std::collections::HashMap;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable::{Hittable, Triangle};
use crate::light::{self, Light};
use crate::material::{Material, PbrMaterial};
use crate::texture::Texture;

use asset_importer::camera::Camera as AiCamera;
use asset_importer::light::{Light as AiLight, LightType};
use asset_importer::mesh::Mesh as AiMesh;
use asset_importer::node::Node as AiNode;
use asset_importer::postprocess::PostProcessSteps;
use asset_importer::scene::Scene as AiScene;
use glam::{DMat4, DVec3, Mat4, Vec2, Vec3};

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
    materials: Vec<Rc<dyn Material>>,
}

impl Scene {
    pub fn import(file_path: &str) -> Self {
        let ai_scene = AiScene::from_file_with_flags(
            file_path,
            PostProcessSteps::TRIANGULATE,
        )
        .unwrap();

        let mut result = Scene {
            hittables: Vec::new(),
            lights: Vec::new(),
            camera: Camera::default(),
            materials: Vec::new(),
        };

        result.import_materials(&ai_scene);

        // build name -> camera/light lookup tables
        let cameras: HashMap<String, AiCamera> = ai_scene
            .cameras()
            .map(|c| (c.name(), c))
            .collect();
        let lights: HashMap<String, AiLight> = ai_scene
            .lights()
            .map(|l| (l.name(), l))
            .collect();

        if let Some(root) = ai_scene.root_node() {
            result.process_node(&root, DMat4::IDENTITY, &ai_scene, &cameras, &lights);
        } else {
            panic!("Scene has no root node");
        }

        result
    }

    fn process_node(
        &mut self,
        node: &AiNode,
        parent_transform: DMat4,
        ai_scene: &AiScene,
        cameras: &HashMap<String, AiCamera>,
        lights: &HashMap<String, AiLight>,
    ) {
        let transform = parent_transform * Mat4::from(node.transformation()).as_dmat4();
        let name = node.name();

        // mesh
        for mesh_idx in node.mesh_indices_iter() {
            if let Some(mesh) = ai_scene.mesh(mesh_idx) {
                self.build_triangles(&mesh, transform);
            }
        }

        // camera
        if let Some(ai_camera) = cameras.get(&name) {
            self.camera = Self::build_camera(ai_camera, transform);
        }

        // light
        if let Some(ai_light) = lights.get(&name) {
            if let Some(light) = Self::build_light(ai_light, transform) {
                self.lights.push(light);
            }
        }

        for child in node.children() {
            self.process_node(&child, transform, ai_scene, cameras, lights);
        }
    }

    fn import_materials(&mut self, ai_scene: &AiScene) {
        for ai_mat in ai_scene.materials() {
            let metallic = ai_mat.metallic_factor().unwrap_or(0.0);
            let roughness = ai_mat.roughness_factor().unwrap_or(0.5);

            let albedo = ai_mat
                .base_color()
                .map(|c| Vec3::new(c.x, c.y, c.z))
                .or_else(|| ai_mat.diffuse_color().map(Vec3::from))
                .unwrap();

            let albedo_tex = Texture::plain(albedo); // todo: load actual texture
            let surface_tex = Texture::plain(Vec3::new(roughness, metallic, 0.0));
            let material = Rc::new(PbrMaterial::new(albedo_tex, surface_tex));
            self.materials.push(material);
        }
    }

    fn build_triangles(&mut self, mesh: &AiMesh, transform: DMat4) {
        let material = self.materials[mesh.material_index()].clone();

        let positions: Vec<DVec3> = mesh
            .vertices_iter()
            .map(|v| transform.transform_point3(Vec3::from(v).as_dvec3()))
            .collect();

        let normals: Option<Vec<DVec3>> = mesh.has_normals().then(|| {
            mesh.normals_iter()
                .map(|n| transform.transform_vector3(Vec3::from(n).as_dvec3()).normalize())
                .collect()
        });

        let tex_coords: Vec<Vec2> = if mesh.has_texture_coords(0) {
            mesh.texture_coords_iter2(0)
                .map(|uv| Vec2::new(uv.x, uv.y))
                .collect()
        } else {
            Vec::new()
        };

        for tri in mesh.triangles_iter() {
            let vertices = [
                positions[tri[0] as usize],
                positions[tri[1] as usize],
                positions[tri[2] as usize],
            ];
            let normal = match &normals {
                Some(normals) => [
                    normals[tri[0] as usize],
                    normals[tri[1] as usize],
                    normals[tri[2] as usize],
                ],
                None => {
                    let v1 = vertices[1] - vertices[0];
                    let v2 = vertices[2] - vertices[0];
                    [DVec3::cross(v1, v2).normalize(); 3]
                }
            };
            let uvs = if tex_coords.is_empty() {
                [Vec2::ZERO; 3]
            } else {
                [
                    tex_coords[tri[0] as usize],
                    tex_coords[tri[1] as usize],
                    tex_coords[tri[2] as usize],
                ]
            };
            let triangle = Triangle::new(vertices, normal, uvs, material.clone());
            self.hittables.push(Box::new(triangle));
        }
    }

    fn build_camera(ai_camera: &AiCamera, transform: DMat4) -> Camera {
        let local_pos = Vec3::from(ai_camera.position()).as_dvec3();
        let local_look_at = Vec3::from(ai_camera.look_at()).as_dvec3();
        let local_up = Vec3::from(ai_camera.up()).as_dvec3();

        let pos = transform.transform_point3(local_pos);
        let forward = transform.transform_vector3(local_look_at).normalize();
        let up = transform.transform_vector3(local_up).normalize();

        let hfov = ai_camera.horizontal_fov();
        let aspect = match ai_camera.aspect() {
            0.0 => 4.0 / 3.0,
            a => a as f64,
        };
        // convert horizontal fov to vertical fov
        let vfov = (2.0 * (hfov / 2.0).tan() / aspect as f32).atan();

        Camera {
            pos,
            forward,
            up: Some(up),
            aspect_ratio: aspect,
            fov: vfov.to_degrees() as f64,
            ..Default::default()
        }
    }

    fn build_light(ai_light: &AiLight, transform: DMat4) -> Option<Box<dyn Light>> {
        let color = Vec3::from(ai_light.color_diffuse());

        let light: Box<dyn Light> = match ai_light.light_type() {
            LightType::Directional => Box::new(light::Directional {
                color,
                dir: transform
                    .transform_vector3(Vec3::from(ai_light.direction()).as_dvec3())
                    .normalize(),
            }),
            LightType::Point => Box::new(light::Point {
                color,
                pos: transform.transform_point3(Vec3::from(ai_light.position()).as_dvec3()),
            }),
            LightType::Spot => Box::new(light::Spot {
                color,
                pos: transform.transform_point3(Vec3::from(ai_light.position()).as_dvec3()),
                dir: transform
                    .transform_vector3(Vec3::from(ai_light.direction()).as_dvec3())
                    .normalize(),
                inner_angle: ai_light.angle_inner_cone(),
                outer_angle: ai_light.angle_outer_cone(),
            }),
            _ => todo!(),
        };
        Some(light)
    }
}

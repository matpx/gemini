use std::collections::{hash_map::Entry, HashMap};

use crate::{
    components::{MeshComponent, TransformComponent},
    gpu::{Geometry, UniformContext, Vertex},
    scene::Scene,
};
use gltf::Node;
use itertools::izip;
use slotmap::DefaultKey;
use wgpu::Device;

use super::LoaderError;

fn load_node(
    device: &Device,
    uniforms: &UniformContext,
    scene: &mut Scene,
    node: &Node,
    buffers: &[gltf::buffer::Data],
    _images: &[gltf::image::Data],
    parent: Option<DefaultKey>,
    known_meshes: &mut HashMap<usize, usize>,
) -> Result<DefaultKey, Box<dyn std::error::Error>> {
    let gltf_transform = node.transform().decomposed();
    let transform = TransformComponent {
        translation: gltf_transform.0.into(),
        rotation: gltf_transform.1.into(),
        scale: gltf_transform.2.into(),
        parent,
        ..Default::default()
    };

    let entity;

    if let Some(mesh) = node.mesh() {
        let geometry_id = match known_meshes.entry(mesh.index()) {
            Entry::Occupied(v) => *v.get(),
            Entry::Vacant(v) => {
                let mut index_data: Vec<u32> = Vec::new();
                let mut vertex_data: Vec<Vertex> = Vec::new();

                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|b| Some(&buffers[b.index()]));

                    let index_data_iter = reader.read_indices().ok_or(LoaderError)?;
                    let vertex_data_iter = reader.read_positions().ok_or(LoaderError)?;
                    let uv_data_iter = reader.read_tex_coords(0).ok_or(LoaderError {})?;
                    let normal_data_iter = reader.read_normals().ok_or(LoaderError {})?;

                    for index in index_data_iter.into_u32() {
                        index_data.push(index);
                    }

                    for (position, uv, normal) in
                        izip!(vertex_data_iter, uv_data_iter.into_f32(), normal_data_iter)
                    {
                        vertex_data.push(Vertex {
                            position,
                            uv,
                            normal,
                        });
                    }
                }

                let geometry = Geometry::new(
                    &device,
                    &uniforms.local_bind_group_layout,
                    &vertex_data,
                    &index_data,
                );

                let id = scene.geometries.insert(geometry);

                v.insert(id);

                id
            }
        };

        entity = scene.create_entity(transform);

        scene.components.meshes.insert(
            entity,
            MeshComponent {
                geometry_id,
                pipeline_id: 0,
            },
        );
    } else {
        entity = scene.create_entity(transform);
    }

    for child_node in node.children() {
        load_node(
            device,
            uniforms,
            scene,
            &child_node,
            buffers,
            _images,
            Some(entity),
            known_meshes,
        )?;
    }

    Ok(entity)
}

pub fn load_gltf(
    device: &Device,
    uniforms: &UniformContext,
    scene: &mut Scene,
    path: &str,
) -> Result<DefaultKey, Box<dyn std::error::Error>> {
    let (document, buffers, images) = gltf::import(path)?;
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    let root_entity = scene.create_entity(TransformComponent::default());

    let mut known_meshes = HashMap::new();

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(
                device,
                uniforms,
                scene,
                &node,
                &buffers,
                &images,
                Some(root_entity),
                &mut known_meshes,
            )?;
        }
    }

    Ok(root_entity)
}

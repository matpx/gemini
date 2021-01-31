use super::LoaderError;
use crate::{
    components::{MeshComponent, MeshPrimitive, TransformComponent},
    gpu::{Context, Geometry, Vertex},
    scene::Scene,
};
use gltf::{Node, Primitive};
use itertools::izip;
use slotmap::DefaultKey;
use std::collections::{hash_map::Entry, HashMap};

fn load_primitive(
    context: &Context,
    buffers: &[gltf::buffer::Data],
    primitive: &Primitive,
) -> Result<Geometry, Box<dyn std::error::Error>> {
    let mut index_data: Vec<u32> = Vec::new();
    let mut vertex_data: Vec<Vertex> = Vec::new();

    let reader = primitive.reader(|b| Some(&buffers[b.index()]));

    let index_data_iter = reader.read_indices().ok_or(LoaderError)?;
    let vertex_data_iter = reader.read_positions().ok_or(LoaderError)?;
    let uv_data_iter = reader.read_tex_coords(0).ok_or(LoaderError {})?;
    let normal_data_iter = reader.read_normals().ok_or(LoaderError {})?;

    for index in index_data_iter.into_u32() {
        index_data.push(index);
    }

    for (position, uv, normal) in izip!(vertex_data_iter, uv_data_iter.into_f32(), normal_data_iter)
    {
        vertex_data.push(Vertex {
            position,
            uv,
            normal,
        });
    }

    Ok(Geometry::new(&context.device, &vertex_data, &index_data))
}

fn load_node(
    context: &Context,
    scene: &mut Scene,
    node: &Node,
    buffers: &[gltf::buffer::Data],
    _images: &[gltf::image::Data],
    parent: Option<DefaultKey>,
    known_meshes: &mut HashMap<usize, DefaultKey>,
) -> Result<DefaultKey, Box<dyn std::error::Error>> {
    let gltf_transform = node.transform().decomposed();
    let transform = TransformComponent {
        translation: gltf_transform.0.into(),
        rotation: gltf_transform.1.into(),
        scale: gltf_transform.2.into(),
        parent,
        ..Default::default()
    };

    let entity = scene.create_entity(transform);
    if let Some(mesh) = node.mesh() {
        let mesh_component = match known_meshes.entry(mesh.index()) {
            Entry::Occupied(v) => {
                let source_mesh_component = scene.components.meshes.get(*v.get()).unwrap();

                let mut mc = MeshComponent::default();

                for ref_prim in &source_mesh_component.primitives {
                    mc.primitives.push(MeshPrimitive::new(
                        &context.device,
                        &context.uniform_layouts.local_bind_group_layout,
                        ref_prim.geometry_id,
                        ref_prim.pipeline_id,
                    ));
                }

                mc
            }
            Entry::Vacant(v) => {
                let mut mc = MeshComponent::default();

                for primitive in mesh.primitives() {
                    let geometry_id = scene
                        .geometries
                        .insert(load_primitive(context, buffers, &primitive)?);

                    let primitive = MeshPrimitive::new(
                        &context.device,
                        &context.uniform_layouts.local_bind_group_layout,
                        geometry_id,
                        0,
                    );

                    mc.primitives.push(primitive);
                }

                v.insert(entity);
                mc
            }
        };

        scene.components.meshes.insert(entity, mesh_component);
    }

    for child_node in node.children() {
        load_node(
            context,
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
    context: &Context,
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
                context,
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

use crate::{
    components::{MaterialComponent, MeshComponent, TransformComponent},
    gpu::{Context, Mesh, Vertex},
    scene::Scene,
};
use gltf::Node;
use legion::Entity;

use super::LoaderError;

fn load_node(
    context: &Context,
    scene: &mut Scene,
    node: &Node,
    buffers: &[gltf::buffer::Data],
    _images: &[gltf::image::Data],
    parent: Option<Entity>,
) -> Result<Entity, Box<dyn std::error::Error>> {
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
        let mut index_data: Vec<u32> = Vec::new();
        let mut vertex_data: Vec<Vertex> = Vec::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|b| Some(&buffers[b.index()]));

            let index_data_iter = reader.read_indices().ok_or(LoaderError)?;
            let vertex_data_iter = reader.read_positions().ok_or(LoaderError)?;

            for index in index_data_iter.into_u32() {
                index_data.push(index);
            }

            for position in vertex_data_iter {
                vertex_data.push(Vertex { pos: position });
            }
        }

        let mesh = Mesh::new(
            &context.device,
            &context.local_bind_group_layout,
            &vertex_data,
            &index_data,
        );

        let mesh_id = scene.meshes.insert(mesh);

        entity = scene.world.push((
            MeshComponent { mesh_id },
            MaterialComponent { material_id: 0 },
            transform,
        ));
    } else {
        entity = scene.world.push((transform,));
    }

    for child_node in node.children() {
        load_node(context, scene, &child_node, buffers, _images, Some(entity))?;
    }

    Ok(entity)
}

pub fn load_gltf(
    context: &Context,
    scene: &mut Scene,
    path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(context, scene, &node, &buffers, &images, None)?;
        }
    }

    Ok(document.nodes().len())
}

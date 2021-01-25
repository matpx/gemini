use crate::{
    components::{MaterialComponent, MeshComponent, TransformComponent},
    gpu::{Context, Mesh, Vertex},
    scene::Scene,
};
use gltf::Node;
use legion::Entity;

use super::LoaderError;

fn load_node(
    node: &Node,
    scene: &mut Scene,
    buffers: &Vec<gltf::buffer::Data>,
    _images: &Vec<gltf::image::Data>,
    context: &Context,
) -> Result<Entity, Box<dyn std::error::Error>> {
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

        Ok(scene.world.push((
            MeshComponent { mesh_id },
            MaterialComponent { material_id: 0 },
            TransformComponent::default(),
        )))
    } else {
        Ok(scene.world.push((TransformComponent::default(),)))
    }
}

pub fn load_gltf(
    path: &str,
    scene: &mut Scene,
    context: &Context,
) -> Result<usize, Box<dyn std::error::Error>> {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(&node, scene, &buffers, &images, context)?;
        }
    }

    Ok(document.nodes().len())
}

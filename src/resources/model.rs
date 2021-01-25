use std::default;

use crate::{
    components::{MaterialComponent, MeshComponent, TransformComponent},
    gpu::{Context, Mesh, Vertex},
    scene::Scene,
};
use gltf::Node;

fn load_node(
    node: &Node,
    scene: &mut Scene,
    buffers: &Vec<gltf::buffer::Data>,
    images: &Vec<gltf::image::Data>,
    context: &Context,
) {
    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|b| Some(&buffers[b.index()]));

            let mut index_data: Vec<u32> = Vec::new();

            for index in reader.read_indices().unwrap().into_u32() {
                index_data.push(index);
            }

            let mut vertex_data: Vec<Vertex> = Vec::new();

            for position in reader.read_positions().unwrap() {
                vertex_data.push(Vertex { pos: position });
            }

            let mesh = Mesh::new(
                &context.device,
                &context.local_bind_group_layout,
                &vertex_data,
                &index_data,
            );

            let mesh_id = scene.meshes.insert(mesh);

            scene.world.push((
                MeshComponent { mesh_id },
                MaterialComponent { material_id: 0 },
                TransformComponent::default(),
            ));
        }
    }
}

pub fn load_gltf(path: &str, scene: &mut Scene, context: &Context) {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(&node, scene, &buffers, &images, context);
        }
    }
}

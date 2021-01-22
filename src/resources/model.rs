use crate::scene::Scene;
use gltf::Node;

fn load_node(
    node: &Node,
    scene: &mut Scene,
    buffers: &Vec<gltf::buffer::Data>,
    images: &Vec<gltf::image::Data>,
) {
    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|b| Some(&buffers[b.index()]));
        }
    }
}

pub fn load_gltf(path: &str, scene: &mut Scene) {
    let (document, buffers, images) = gltf::import(path).unwrap();
    assert_eq!(buffers.len(), document.buffers().count());
    assert_eq!(images.len(), document.images().count());

    for document_scene in document.scenes() {
        for node in document_scene.nodes() {
            load_node(&node, scene, &buffers, &images);
        }
    }
}

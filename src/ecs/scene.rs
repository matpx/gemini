use super::components::*;
use crate::gpu::{Mesh, RenderPipeline};
use slab::Slab;
use std::collections::HashMap;

pub struct Scene {
    meshes: Slab<Mesh>,
    pipelines: Slab<RenderPipeline>,

    mesh_components: HashMap<usize, MeshComponent>,
    material_components: HashMap<usize, MaterialComponent>,
    transform_components: HashMap<usize, TransformComponent>,

    entitiy_counter: usize,
}

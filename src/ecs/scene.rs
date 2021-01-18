use super::Entity;
use crate::gpu::{Mesh, RenderPipeline};
use slab::Slab;

#[derive(Default)]
pub struct Scene {
    pub meshes: Slab<Mesh>,
    pub pipelines: Slab<RenderPipeline>,

    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }
}

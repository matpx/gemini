use crate::gpu::{Mesh, RenderPipeline};
use legion::World;
use slab::Slab;

#[derive(Default)]
pub struct Scene {
    pub meshes: Slab<Mesh>,
    pub pipelines: Slab<RenderPipeline>,

    pub world: World,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }
}

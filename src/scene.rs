use crate::gpu::{Geometry, Pipeline};
use legion::World;
use slab::Slab;

#[derive(Default)]
pub struct Scene {
    pub meshes: Slab<Geometry>,
    pub pipelines: Slab<Pipeline>,

    pub world: World,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }
}

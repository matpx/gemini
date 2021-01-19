use crate::gpu::{Material, Mesh};
use legion::World;
use slab::Slab;

#[derive(Default)]
pub struct Scene {
    pub meshes: Slab<Mesh>,
    pub materials: Slab<Material>,

    pub world: World,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }
}

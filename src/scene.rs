use crate::{
    components::{CameraComponent, MeshComponent, PlayerComponent, TransformComponent},
    gpu::{Geometry, Pipeline},
};
use slab::Slab;
use slotmap::{DefaultKey, HopSlotMap, SecondaryMap};

#[derive(Default)]
pub struct Components {
    pub transforms: HopSlotMap<DefaultKey, TransformComponent>,
    pub transforms_sorted: Vec<DefaultKey>,
    pub meshes: SecondaryMap<DefaultKey, MeshComponent>,
    pub cameras: SecondaryMap<DefaultKey, CameraComponent>,
    pub players: SecondaryMap<DefaultKey, PlayerComponent>,
}

#[derive(Default)]
pub struct Scene {
    pub geometries: Slab<Geometry>,
    pub pipelines: Slab<Pipeline>,

    pub components: Components,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_entity(&mut self, transform: TransformComponent) -> DefaultKey {
        let key = self.components.transforms.insert(transform);

        self.components.transforms_sorted.push(key);

        key
    }
}

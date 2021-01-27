use crate::{
    components::{CameraComponent, MeshComponent, TransformComponent},
    gpu::{Geometry, Pipeline},
};
use slab::Slab;
use std::{collections::HashMap, usize};

#[derive(Default)]
pub struct Components {
    pub meshes: HashMap<usize, MeshComponent>,
    pub transforms: HashMap<usize, TransformComponent>,
    pub cameras: HashMap<usize, CameraComponent>,
}

#[derive(Default)]
pub struct Scene {
    pub geometries: Slab<Geometry>,
    pub pipelines: Slab<Pipeline>,

    pub components: Components,

    entity_counter: usize,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_entity(&mut self) -> usize {
        let new_id = self.entity_counter;

        self.entity_counter = (self.entity_counter + 1) % std::usize::MAX;

        new_id
    }
}

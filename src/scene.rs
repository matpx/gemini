use crate::{
    components::{CameraComponent, MeshComponent, TransformComponent},
    gpu::{Geometry, Pipeline},
};
use slab::Slab;
use std::usize;

pub type Entity = usize;

#[derive(Default)]
pub struct Storage<T: Copy + Default> {
    data: Vec<T>,
}

impl<T: Copy + Default> Storage<T> {
    pub fn insert(&mut self, entity_id: Entity, value: T) {
        if self.data.len() < entity_id + 1 {
            self.data.resize(entity_id + 1, Default::default());

            self.data[entity_id] = value;
        }
    }

    pub fn get(&self, entity_id: Entity) -> Option<&T> {
        if entity_id < self.data.len() {
            return Some(&self.data[entity_id]);
        }

        None
    }

    pub fn get_mut(&mut self, entity_id: Entity) -> Option<&mut T> {
        if entity_id < self.data.len() {
            return Some(&mut self.data[entity_id]);
        }

        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

#[derive(Default)]
pub struct Components {
    pub meshes: Storage<MeshComponent>,
    pub transforms: Storage<TransformComponent>,
    pub cameras: Storage<CameraComponent>,
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

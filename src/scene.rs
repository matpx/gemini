use crate::{
    components::{CameraComponent, MeshComponent, TransformComponent},
    gpu::{Geometry, Pipeline},
};
use slab::Slab;
use std::usize;

pub type Entity = usize;

#[derive(Default)]
pub struct Storage<T: Copy + Default> {
    data: Vec<Option<T>>,
}

impl<'a, T: Copy + Default> Storage<T> {
    pub fn insert(&mut self, entity_id: Entity, value: T) {
        if self.data.len() < entity_id + 1 {
            self.data.resize(entity_id + 1, None);

            self.data[entity_id] = Some(value);
        }
    }

    pub fn get(&self, entity_id: Entity) -> Option<&T> {
        if entity_id < self.data.len() {
            if let Some(value) = &self.data[entity_id] {
                return Some(value);
            }
        }

        None
    }

    pub fn get_mut(&mut self, entity_id: Entity) -> Option<&mut T> {
        if entity_id < self.data.len() {
            if let Some(value) = &mut self.data[entity_id] {
                return Some(value);
            }
        }

        None
    }

    pub fn iter(&self) -> StorageIterator<'_, T> {
        StorageIterator {
            storage: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> StorageIteratorMut<'_, T> {
        StorageIteratorMut {
            storage: self,
            index: 0,
        }
    }
}

pub struct StorageIterator<'a, T: Copy + Default> {
    storage: &'a Storage<T>,
    index: usize,
}

impl<'a, T: Copy + Default + 'a> Iterator for StorageIterator<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.storage.data.len() {
                return None;
            }

            let index = self.index;

            self.index += 1;

            if let Some(item) = &self.storage.data[index] {
                return Some((index, item));
            }
        }
    }
}

pub struct StorageIteratorMut<'a, T: Copy + Default> {
    storage: &'a mut Storage<T>,
    index: usize,
}

impl<'a, T: Copy + Default + 'a> Iterator for StorageIteratorMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.storage.data.len() {
                return None;
            }

            let index = self.index;

            self.index += 1;

            if let Some(item) = &self.storage.data[index] {
                unsafe {
                    return Some((index, &mut *((item as *const T) as *mut T)));
                }
            }
        }
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

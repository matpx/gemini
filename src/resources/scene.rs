use super::model::load_gltf;
use crate::{
    components::{CameraComponent, MeshComponent, PlayerComponent, TransformComponent},
    gpu::Context,
    resources::{
        manager::ResourceManager,
        map::{Map, Node},
        prefab::Prefab,
    },
};
use slotmap::{DefaultKey, HopSlotMap, SecondaryMap};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Scene {
    pub transforms: HopSlotMap<DefaultKey, TransformComponent>,
    pub transforms_sorted: Vec<DefaultKey>,
    pub meshes: SecondaryMap<DefaultKey, MeshComponent>,
    pub cameras: SecondaryMap<DefaultKey, CameraComponent>,
    pub players: SecondaryMap<DefaultKey, PlayerComponent>,
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_entity(&mut self, transform: TransformComponent) -> DefaultKey {
        let key = self.transforms.insert(transform);

        self.transforms_sorted.push(key);

        key
    }

    pub fn copy_from(&mut self, other: &Scene, root: &DefaultKey) -> DefaultKey {
        let mut parent_mapping = HashMap::<DefaultKey, DefaultKey>::new();

        for (other_key, transform) in &other.transforms {
            let self_key;
            if let Some(other_parent) = &transform.parent {
                self_key = self.transforms.insert(TransformComponent {
                    parent: Some(*parent_mapping.get(other_parent).unwrap()),
                    ..*transform
                });
            } else {
                self_key = self.transforms.insert(*transform);
            }

            self.transforms_sorted.push(self_key);

            parent_mapping.insert(other_key, self_key);

            if let Some(other_mesh) = other.meshes.get(other_key) {
                self.meshes.insert(self_key, other_mesh.clone());
            }

            if let Some(other_camera) = other.cameras.get(other_key) {
                self.cameras.insert(self_key, *other_camera);
            }

            if let Some(other_player) = other.players.get(other_key) {
                self.players.insert(self_key, *other_player);
            }
        }

        *parent_mapping.get(root).unwrap()
    }

    fn load_node(
        &mut self,
        context: &Context,
        resource_manager: &mut ResourceManager,
        prefabs: &[Prefab],
        node: &Node,
        parent: Option<DefaultKey>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transform = TransformComponent {
            translation: node.translation,
            rotation: node.rotation,
            scale: node.scale,
            parent,
            ..Default::default()
        };

        let new_id = if let Some(prefab_id) = node.prefab_id {
            let prefab = &prefabs[prefab_id];
            let new_root = self.copy_from(&prefab.scene, &prefab.root);

            *self.transforms.get_mut(new_root).unwrap() = transform;

            new_root
        } else {
            self.create_entity(transform)
        };

        for child in &node.children {
            self.load_node(context, resource_manager, prefabs, child, Some(new_id))?;
        }

        Ok(())
    }

    pub fn load_map(
        &mut self,
        context: &Context,
        resource_manager: &mut ResourceManager,
        map: &Map,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut prefabs = Vec::new();

        for prefab_id in &map.prefabs {
            prefabs.push(load_gltf(context, resource_manager, prefab_id)?);
        }

        self.load_node(context, resource_manager, &prefabs, &map.root, None)?;

        Ok(())
    }
}

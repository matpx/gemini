use super::LoaderError;
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

    pub fn copy_from(&mut self, other: &Scene) -> HashMap<DefaultKey, DefaultKey> {
        let mut parent_mapping = HashMap::<DefaultKey, DefaultKey>::new();

        for (other_key, transform) in &other.components.transforms {
            let self_key;
            if let Some(other_parent) = &transform.parent {
                self_key = self.components.transforms.insert(TransformComponent {
                    parent: Some(*parent_mapping.get(other_parent).unwrap()),
                    ..*transform
                });
            } else {
                self_key = self.components.transforms.insert(*transform);
            }

            self.components.transforms_sorted.push(self_key);

            parent_mapping.insert(other_key, self_key);

            if let Some(other_mesh) = other.components.meshes.get(other_key) {
                self.components.meshes.insert(self_key, other_mesh.clone());
            }

            if let Some(other_camera) = other.components.cameras.get(other_key) {
                self.components.cameras.insert(self_key, *other_camera);
            }

            if let Some(other_player) = other.components.players.get(other_key) {
                self.components.players.insert(self_key, *other_player);
            }
        }

        parent_mapping
    }

    fn load_node_subtree(
        &mut self,
        context: &Context,
        resource_manager: &mut ResourceManager,
        prefabs: &[Prefab],
        node: &Node,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(model) = node.prefab_id {
            let prefab = prefabs.get(model).ok_or(LoaderError)?;
            let _mapping = self.copy_from(&prefab.scene);
        }

        for child_node in &node.children {
            self.load_node_subtree(context, resource_manager, &prefabs, child_node)?;
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

        for model_id in &map.prefabs {
            prefabs.push(Prefab::from_gltf(context, resource_manager, model_id)?);
        }

        self.load_node_subtree(context, resource_manager, &prefabs, &map.root)?;

        Ok(())
    }
}

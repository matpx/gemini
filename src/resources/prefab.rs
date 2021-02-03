use crate::gpu::Context;
use slotmap::DefaultKey;

use super::{manager::ResourceManager, model::load_gltf, scene::Scene};

pub struct Prefab {
    pub root: DefaultKey,
    pub scene: Scene,
}

impl Prefab {
    pub fn from_gltf(
        context: &Context,
        resource_manager: &mut ResourceManager,
        path: &String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut scene = Scene::new();
        let root = load_gltf(context, resource_manager, &mut scene, path)?;

        Ok(Self { root, scene })
    }
}

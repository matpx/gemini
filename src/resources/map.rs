use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub prefab_id: Option<usize>,
    pub children: Vec<Node>,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            prefab_id: None,
            children: Vec::new(),
            translation: glam::Vec3::zero(),
            rotation: glam::Quat::identity(),
            scale: glam::Vec3::one(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Map {
    pub prefabs: Vec<String>,
    pub root: Node,
}

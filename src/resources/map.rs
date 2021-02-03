use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Node {
    pub prefab_id: Option<usize>,
    pub children: Vec<Node>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Map {
    pub prefabs: Vec<String>,
    pub root: Node,
}

use super::scene::Scene;
use slotmap::DefaultKey;

#[derive(Debug, Default)]
pub struct Prefab {
    pub root: DefaultKey,
    pub scene: Scene,
}

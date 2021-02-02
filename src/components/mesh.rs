use super::MeshPrimitive;
use smallvec::SmallVec;

#[derive(Debug, Default, Clone)]
pub struct MeshComponent {
    pub primitives: SmallVec<[MeshPrimitive; 4]>,
}

impl MeshComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

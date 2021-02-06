use super::material::PbrMaterial;

#[derive(Debug, Default, Clone, Copy)]
pub struct MeshPrimitive {
    pub geometry_id: usize,
    pub pipeline_id: usize,
    pub material: PbrMaterial,
}

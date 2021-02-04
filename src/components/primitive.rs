#[derive(Debug, Default, Clone)]
pub struct MeshPrimitive {
    pub geometry_id: usize,
    pub pipeline_id: usize,
    pub color_texture: Option<usize>,
}

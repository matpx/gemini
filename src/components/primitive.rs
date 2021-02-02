#[derive(Debug, Default, Clone)]
pub struct MeshPrimitive {
    pub geometry_id: usize,
    pub pipeline_id: usize,
}

impl MeshPrimitive {
    pub fn new(geometry_id: usize, pipeline_id: usize) -> Self {
        Self {
            geometry_id,
            pipeline_id,
        }
    }
}

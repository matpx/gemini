pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

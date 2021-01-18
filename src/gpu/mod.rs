pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub local_bind_group: wgpu::BindGroup,
}

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

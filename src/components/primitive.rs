use crate::gpu::uniform::{PrimitiveUniformData, UniformLayouts};
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device};
#[derive(Debug)]
pub struct MeshPrimitive {
    pub geometry_id: usize,
    pub pipeline_id: usize,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl MeshPrimitive {
    pub fn new(
        device: &Device,
        uniform_layouts: &UniformLayouts,
        geometry_id: usize,
        pipeline_id: usize,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&PrimitiveUniformData::default()),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.primitive_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: None,
        });

        Self {
            geometry_id,
            pipeline_id,
            buffer,
            bind_group,
        }
    }
}

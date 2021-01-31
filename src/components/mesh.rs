use crate::gpu::uniform::TransformUniformData;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

#[derive(Default, Debug)]
pub struct MeshComponent {
    pub primitives: Vec<MeshPrimitive>,
}

#[derive(Debug)]
pub struct MeshPrimitive {
    pub geometry_id: usize,
    pub pipeline_id: usize,
    pub local_buffer: Buffer,
    pub local_bind_group: BindGroup,
}

impl MeshPrimitive {
    pub fn new(
        device: &Device,
        local_bind_group_layout: &BindGroupLayout,
        geometry_id: usize,
        pipeline_id: usize,
    ) -> Self {
        let local_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&TransformUniformData {
                model: glam::Mat4::identity(),
            }),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let local_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &local_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(local_buffer.slice(..)),
            }],
            label: None,
        });

        Self {
            geometry_id,
            pipeline_id,
            local_buffer,
            local_bind_group,
        }
    }
}

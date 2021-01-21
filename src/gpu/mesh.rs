use super::{EntityUniform, Vertex};
use wgpu::{util::DeviceExt, BindGroupLayout, Buffer, Device};

pub struct Mesh {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub local_bind_group: wgpu::BindGroup,
    pub local_buffer: Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(
        device: &Device,
        local_bind_group_layout: &BindGroupLayout,
        vertex_data: &[Vertex; 4],
        index_data: &[u16],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(index_data),
            usage: wgpu::BufferUsage::INDEX,
        });

        let local_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&EntityUniform {
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
            vertex_buffer,
            index_buffer,
            local_bind_group,
            local_buffer,
            index_count: index_data.len() as u32,
        }
    }
}

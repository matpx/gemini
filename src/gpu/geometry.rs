use super::Vertex;
use wgpu::{util::DeviceExt, Device};

pub struct Geometry {
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Geometry {
    pub fn new(device: &Device, vertex_data: &[Vertex], index_data: &[u32]) -> Self {
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

        Self {
            vertex_buffer,
            index_buffer,
            index_count: index_data.len() as u32,
        }
    }
}

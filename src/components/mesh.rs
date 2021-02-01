use super::MeshPrimitive;
use crate::gpu::uniform::{TransformUniformData, UniformLayouts};
use smallvec::SmallVec;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device};

#[derive(Debug)]
pub struct MeshComponent {
    pub primitives: SmallVec<[MeshPrimitive; 4]>,
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl MeshComponent {
    pub fn new(device: &Device, uniform_layouts: &UniformLayouts) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&TransformUniformData::default()),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(
                        std::mem::size_of::<TransformUniformData>() as wgpu::BufferAddress
                    ),
                },
            }],
            label: None,
        });

        Self {
            primitives: SmallVec::new(),
            buffer,
            bind_group,
        }
    }
}

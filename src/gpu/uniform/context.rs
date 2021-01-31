use super::{CameraUniformData, UniformLayouts};
use crate::gpu::DEPTH_FORMAT;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, TextureView};
use winit::dpi::PhysicalSize;

pub struct UniformContext {
    pub global_bind_group: BindGroup,
    pub global_uniform_buffer: Buffer,
    pub depth_view: TextureView,
}

impl UniformContext {
    pub fn new(
        device: &Device,
        uniform_layouts: &UniformLayouts,
        size: &PhysicalSize<u32>,
    ) -> Self {
        let global_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&CameraUniformData {
                view_proj: glam::Mat4::identity(),
            }),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.global_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(global_uniform_buffer.slice(..)),
            }],
            label: None,
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            global_bind_group,
            global_uniform_buffer,
            depth_view,
        }
    }
}

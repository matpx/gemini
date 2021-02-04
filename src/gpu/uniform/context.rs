use super::{CameraUniformData, PrimitiveUniformData, TransformUniformData, UniformLayouts};
use crate::gpu::{texture, DEPTH_FORMAT};
use texture::Texture;
use wgpu::{util::DeviceExt, BindGroup, Buffer, Device, Queue, TextureView};
use winit::dpi::PhysicalSize;

pub struct UniformContext {
    pub camera_bind_group: BindGroup,
    pub camera_uniform_buffer: Buffer,
    pub transform_bind_group: BindGroup,
    pub transform_uniform_buffer: Buffer,
    pub primitive_bind_group: BindGroup,
    pub primitive_uniform_buffer: Buffer,
    pub depth_view: TextureView,
    pub dummy_texture: Texture,
}

impl UniformContext {
    pub fn new(
        device: &Device,
        queue: &Queue,
        uniform_layouts: &UniformLayouts,
        size: &PhysicalSize<u32>,
    ) -> Self {
        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&CameraUniformData {
                view_proj: glam::Mat4::identity(),
            }),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &camera_uniform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(
                        std::mem::size_of::<CameraUniformData>() as wgpu::BufferAddress
                    ),
                },
            }],
            label: None,
        });

        assert!(std::mem::size_of::<TransformUniformData>() as u64 <= wgpu::BIND_BUFFER_ALIGNMENT);

        let transform_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: super::BUFFER_ENTITIES_NUM * wgpu::BIND_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &transform_uniform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(
                        std::mem::size_of::<TransformUniformData>() as wgpu::BufferAddress
                    ),
                },
            }],
            label: None,
        });

        assert!(std::mem::size_of::<PrimitiveUniformData>() as u64 <= wgpu::BIND_BUFFER_ALIGNMENT);

        let primitive_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: super::BUFFER_ENTITIES_NUM * wgpu::BIND_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let primitive_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.primitive_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &primitive_uniform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(
                        std::mem::size_of::<PrimitiveUniformData>() as wgpu::BufferAddress
                    ),
                },
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
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            label: None,
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let dummy_texture = Texture::new(
            device,
            queue,
            uniform_layouts,
            (32, 32),
            &vec![255; 32 * 32 * 4],
        );

        Self {
            camera_bind_group,
            camera_uniform_buffer,
            transform_bind_group,
            transform_uniform_buffer,
            primitive_bind_group,
            primitive_uniform_buffer,
            depth_view,
            dummy_texture,
        }
    }
}

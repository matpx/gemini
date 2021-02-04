use wgpu::{util::DeviceExt, BindGroup, Device, Queue};

use super::uniform::UniformLayouts;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub bind_group: BindGroup,
}

impl Texture {
    pub fn new(
        device: &Device,
        queue: &Queue,
        uniform_layouts: &UniformLayouts,
        size: (u32, u32),
        format: wgpu::TextureFormat,
        data: &[u8],
    ) -> Self {
        let texture_extent = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth: 1,
        };

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: None,
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            },
            &data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_layouts.color_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: None,
        });

        Self {
            texture,
            bind_group,
        }
    }
}

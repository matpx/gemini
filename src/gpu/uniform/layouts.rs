use super::{CameraUniformData, PrimitiveUniformData, TransformUniformData};
use wgpu::{BindGroupLayout, Device};

pub struct UniformLayouts {
    pub transform_bind_group_layout: BindGroupLayout,
    pub primitive_bind_group_layout: BindGroupLayout,
    pub camera_bind_group_layout: BindGroupLayout,
}

impl UniformLayouts {
    pub fn new(device: &Device) -> Self {
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<CameraUniformData>() as wgpu::BufferAddress,
                        ),
                    },
                    count: None,
                }],
                label: None,
            });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                            TransformUniformData,
                        >()
                            as wgpu::BufferAddress),
                    },
                    count: None,
                }],
                label: None,
            });

        let primitive_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                            PrimitiveUniformData,
                        >()
                            as wgpu::BufferAddress),
                    },
                    count: None,
                }],
                label: None,
            });

        Self {
            transform_bind_group_layout,
            primitive_bind_group_layout,
            camera_bind_group_layout,
        }
    }
}

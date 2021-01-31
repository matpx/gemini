use super::{CameraUniformData, TransformUniformData};
use wgpu::{BindGroupLayout, Device};

pub struct UniformLayouts {
    pub local_bind_group_layout: BindGroupLayout,
    pub global_bind_group_layout: BindGroupLayout,
}

impl UniformLayouts {
    pub fn new(device: &Device) -> Self {
        let global_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<CameraUniformData>() as wgpu::BufferAddress,
                        ),
                    },
                    count: None,
                }],
                label: None,
            });

        let local_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<
                            TransformUniformData,
                        >()
                            as wgpu::BufferAddress),
                    },
                    count: None,
                }],
                label: None,
            });

        Self {
            local_bind_group_layout,
            global_bind_group_layout,
        }
    }
}

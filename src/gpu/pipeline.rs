use std::borrow::Cow;

use super::{uniform::UniformLayouts, Vertex};
use wgpu::{Device, SwapChainDescriptor};

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        uniform_layouts: &UniformLayouts,
        sc_desc: &SwapChainDescriptor,
    ) -> Self {
        let flags = wgpu::ShaderFlags::VALIDATION;

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shader/unlit.wgsl"))),
            flags,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &uniform_layouts.camera_bind_group_layout,
                &uniform_layouts.transform_bind_group_layout,
                &uniform_layouts.primitive_bind_group_layout,
                &uniform_layouts.color_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float2,
                            offset: 3 * std::mem::size_of::<f32>() as u64,
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float3,
                            offset: 5 * std::mem::size_of::<f32>() as u64,
                            shader_location: 2,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[sc_desc.format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: super::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState::default(),
        });

        Self { pipeline }
    }
}

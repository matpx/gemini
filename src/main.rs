use components::TransformComponent;
use components::*;
use gpu::{Context, EntityUniform, Material, Vertex};
use legion::*;
use wgpu::{util::DeviceExt, TextureFormat};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod components;
mod gpu;
mod scene;

async fn run(event_loop: EventLoop<()>, window: Window, swapchain_format: TextureFormat) {
    let mut context = Context::new(&window, swapchain_format).await;

    let mut scene = scene::Scene::new();

    let vertex_data = [
        Vertex {
            pos: [-0.5, 0.5, 0.0],
        },
        Vertex {
            pos: [0.5, 0.5, 0.0],
        },
        Vertex {
            pos: [0.5, -0.5, 0.0],
        },
        Vertex {
            pos: [-0.5, -0.5, 0.0],
        },
    ];

    let index_data: &[u16] = &[2, 1, 0, 0, 3, 2];

    let vertex_buffer = context
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

    let index_buffer = context
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });

    let entity_uniform_size = std::mem::size_of::<EntityUniform>() as wgpu::BufferAddress;
    let quad_uniform_buffer =
        context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&EntityUniform {
                    model: glam::Mat4::identity(),
                }),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });

    let local_bind_group_layout =
        context
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(entity_uniform_size),
                    },
                    count: None,
                }],
                label: None,
            });

    let local_bind_group = context
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &local_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(quad_uniform_buffer.slice(..)),
            }],
            label: None,
        });

    let mesh = scene.meshes.insert(gpu::Mesh {
        vertex_buffer,
        index_buffer,
        local_bind_group,
    });

    let vs_module = context
        .device
        .create_shader_module(wgpu::include_spirv!("shader/unlit.vert.spv"));
    let fs_module = context
        .device
        .create_shader_module(wgpu::include_spirv!("shader/unlit.frag.spv"));

    let pipeline_layout = context
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&local_bind_group_layout],
            push_constant_ranges: &[],
        });

    let material_id = scene.materials.insert(Material {
        pipeline: context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("main"),
                layout: Some(&pipeline_layout),
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    ..Default::default()
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[context.swap_chain_desc.format.into()],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[wgpu::VertexBufferDescriptor {
                        stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &[wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        }],
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }),
    });

    scene.world.push((
        MeshComponent { mesh_id: mesh },
        MaterialComponent { material_id },
        TransformComponent {
            position: glam::Vec3::zero(),
            scale: glam::Vec3::one(),
            rotation: glam::Quat::identity(),
        },
    ));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                context.swap_chain_desc.width = size.width;
                context.swap_chain_desc.height = size.height;
                context.swap_chain = context
                    .device
                    .create_swap_chain(&context.surface, &context.swap_chain_desc);
            }
            Event::RedrawRequested(_) => {
                let frame = context
                    .swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
                let mut encoder = context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });

                    for (mesh, material) in
                        <(&MeshComponent, &MaterialComponent)>::query().iter(&scene.world)
                    {
                        let gpu_mesh = scene.meshes.get(mesh.mesh_id).unwrap();
                        let gpu_material = scene.materials.get(material.material_id).unwrap();

                        rpass.set_pipeline(&gpu_material.pipeline);
                        rpass.set_bind_group(0, &gpu_mesh.local_bind_group, &[]);
                        rpass.set_index_buffer(gpu_mesh.index_buffer.slice(..));
                        rpass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                        rpass.draw_indexed(0..index_data.len() as u32, 0, 0..1);
                    }
                }

                context.queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window, wgpu::TextureFormat::Bgra8Unorm));
    }
}

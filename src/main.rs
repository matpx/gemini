use components::TransformComponent;
use components::*;
use gpu::{Context, Material, Mesh, Vertex};
use wgpu::TextureFormat;
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

    let local_bind_group_layout =
        context
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<EntityUniform>() as wgpu::BufferAddress,
                        ),
                    },
                    count: None,
                }],
                label: None,
            });

    let mesh = scene.meshes.insert(Mesh::new(
        &context.device,
        &local_bind_group_layout,
        &vertex_data,
        &index_data,
    ));

    let material_id = scene.materials.insert(Material::new(
        &context.device,
        &local_bind_group_layout,
        swapchain_format,
    ));

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
            Event::RedrawRequested(_) => gpu::render(
                &context.device,
                &mut context.swap_chain,
                &context.queue,
                &scene,
            ),
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

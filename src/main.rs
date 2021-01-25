use components::TransformComponent;
use components::*;
use gpu::{Context, Material};
use input::InputManager;
use systems::transform_system;
use wgpu::TextureFormat;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod components;
mod gpu;
mod input;
mod resources;
mod scene;
mod systems;

async fn run(event_loop: EventLoop<()>, window: Window, swapchain_format: TextureFormat) {
    let mut context = Context::new(&window, swapchain_format).await;

    let mut scene = scene::Scene::new();

    let mut input_manager = InputManager::new();

    /*let vertex_data = [
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

    let mesh = scene.meshes.insert(Mesh::new(
        &context.device,
        &context.local_bind_group_layout,
        &vertex_data,
        &index_data,
    ));

    let player_parent = scene.world.push((
        TransformComponent::default(),
        MeshComponent { mesh_id: mesh },
    ));

    scene.world.push((
        MeshComponent { mesh_id: mesh },
        MaterialComponent { material_id },
        TransformComponent {
            parent: Some(player_parent),
            ..Default::default()
        },
    ));*/

    scene.materials.insert(Material::new(
        &context.device,
        &context.global_bind_group_layout,
        &context.local_bind_group_layout,
        swapchain_format,
    ));

    resources::load_gltf(&context, &mut scene, "assets/gltf/monkey.glb").unwrap();

    let camera = scene
        .world
        .push((TransformComponent::default(), CameraComponent::default()));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    context.swap_chain_desc.width = size.width;
                    context.swap_chain_desc.height = size.height;
                    context.swap_chain = context
                        .device
                        .create_swap_chain(&context.surface, &context.swap_chain_desc);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_manager.handle_keyboard_event(input);
                }

                _ => {}
            },
            Event::RedrawRequested(_) => {
                /*scene
                .world
                .entry(player_parent)
                .unwrap()
                .get_component_mut::<TransformComponent>()
                .unwrap()
                .translation
                .x += 0.001;*/

                input_manager.update();

                transform_system(&mut scene.world);

                gpu::render(
                    &context.device,
                    &mut context.swap_chain,
                    &context.queue,
                    &context.global_bind_group,
                    &context.global_uniform_buffer,
                    &scene,
                    camera,
                );
            }
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

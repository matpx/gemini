use components::TransformComponent;
use components::*;
use gpu::{Context, Pipeline};
use input::InputManager;
use legion::*;
use std::f32::consts::PI;
use systems::transform_system;
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

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut context = Context::new(&window).await;

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

    scene.pipelines.insert(Pipeline::new(
        &context.device,
        &context.uniforms.global_bind_group_layout,
        &context.uniforms.local_bind_group_layout,
    ));

    let test_model = resources::load_gltf(
        &context.device,
        &context.uniforms,
        &mut scene,
        "assets/gltf/monkey.glb",
    )
    .unwrap();

    let camera = scene.world.push((
        TransformComponent::default(),
        CameraComponent::new(
            PI / 4.0,
            context.size().width as f32 / context.size().height as f32,
            0.1,
            100.0,
        ),
    ));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    context.resize(size);

                    for camera in <&mut CameraComponent>::query().iter_mut(&mut scene.world) {
                        camera.aspect = size.width as f32 / size.height as f32;
                        camera.update_projection_matrix();
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_manager.handle_keyboard_event(input);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    input_manager.handle_mouse_event(position);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                scene
                    .world
                    .entry(test_model)
                    .unwrap()
                    .get_component_mut::<TransformComponent>()
                    .unwrap()
                    .translation
                    .x += 0.001;

                input_manager.update();

                transform_system(&mut scene.world);

                input_manager.late_update();

                gpu::render(
                    &context.device,
                    &context.queue,
                    &mut context.swap_chain,
                    &context.uniforms,
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
        pollster::block_on(run(event_loop, window));
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
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}

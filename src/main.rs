use components::TransformComponent;
use components::*;
use gpu::{Context, Pipeline};
use input::InputManager;
use resources::{manager::ResourceManager, map::Map, scene::Scene};
use std::f32::consts::PI;
use systems::{PlayerSystem, TransformSystem};
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

mod components;
mod gpu;
mod input;
mod resources;
mod shapes;
mod systems;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut context = Context::new(&window).await;

    let mut scene = Scene::new();

    let mut input_manager = InputManager::new();
    let mut resource_manager = ResourceManager::default();

    let mut test_map = Map::default();
    test_map
        .prefabs
        .push(String::from("assets/gltf/monkey.glb"));
    test_map.root.prefab_id = Some(0);

    scene
        .load_map(&context, &mut resource_manager, &test_map)
        .unwrap();

    resource_manager.pipelines.insert(Pipeline::new(
        &context.device,
        &context.uniform_layouts,
        &context.swap_chain_desc,
    ));

    let player_entity = PlayerSystem::setup(&mut scene);

    let camera = scene.create_entity(TransformComponent {
        parent: Some(player_entity),
        ..Default::default()
    });

    scene.components.cameras.insert(
        camera,
        CameraComponent::new(
            PI / 4.0,
            context.size().width as f32 / context.size().height as f32,
            0.1,
            100.0,
        ),
    );

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

                    for (_, camera) in scene.components.cameras.iter_mut() {
                        camera.aspect = size.width as f32 / size.height as f32;
                        camera.update_projection_matrix();
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key_code) = input.virtual_keycode {
                        if key_code == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    input_manager.handle_keyboard_event(input);
                }

                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    input_manager.handle_mouse_event(delta);
                }
            }

            Event::RedrawRequested(_) => {
                input_manager.update();

                PlayerSystem::update(&mut scene, &input_manager, player_entity);

                TransformSystem::update(&mut scene);

                input_manager.late_update();

                gpu::render(
                    &context.device,
                    &context.queue,
                    &resource_manager,
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
    let window = WindowBuilder::new()
        .with_title("-")
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

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

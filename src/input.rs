use std::{collections::HashSet, ops::MulAssign};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Default, Debug)]
pub struct InputManager {
    pressed_keys: HashSet<VirtualKeyCode>,
    axis: glam::Vec2,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager::default()
    }

    pub fn handle_keyboard_event(&mut self, input: KeyboardInput) {
        if let Some(key) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(key);
                }
                ElementState::Released => {
                    self.pressed_keys.remove(&key);
                }
            };
        }
    }

    pub fn update(&mut self) {
        self.axis.mul_assign(0.9);

        if self.pressed_keys.contains(&VirtualKeyCode::A) {
            self.axis[0] = -1.0;
        } else if self.pressed_keys.contains(&VirtualKeyCode::D) {
            self.axis[0] = 1.0;
        }

        if self.pressed_keys.contains(&VirtualKeyCode::S) {
            self.axis[1] = -1.0;
        } else if self.pressed_keys.contains(&VirtualKeyCode::W) {
            self.axis[1] = 1.0;
        }
    }
}

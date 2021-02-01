use std::{collections::HashSet, ops::Mul, ops::MulAssign};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Default, Debug)]
pub struct InputManager {
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub axis_a: glam::Vec2,
    pub axis_b: glam::Vec2,

    last_mouse_pos: glam::Vec2,
    next_mouse_pos: glam::Vec2,
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

    pub fn handle_mouse_event(&mut self, delta: (f64, f64)) {
        self.next_mouse_pos += glam::vec2(delta.0 as f32, delta.1 as f32);

        let new_axis_b = (self.next_mouse_pos - self.last_mouse_pos).mul(0.1);

        if new_axis_b.length_squared() > 1.0 {
            self.axis_b = new_axis_b.normalize();
        } else {
            self.axis_b = new_axis_b;
        }
    }

    pub fn update(&mut self) {
        if self.pressed_keys.contains(&VirtualKeyCode::A) {
            self.axis_a[0] = -1.0;
        } else if self.pressed_keys.contains(&VirtualKeyCode::D) {
            self.axis_a[0] = 1.0;
        }

        if self.pressed_keys.contains(&VirtualKeyCode::S) {
            self.axis_a[1] = -1.0;
        } else if self.pressed_keys.contains(&VirtualKeyCode::W) {
            self.axis_a[1] = 1.0;
        }
    }

    pub fn late_update(&mut self) {
        self.axis_a.mul_assign(0.9);

        self.last_mouse_pos = self.next_mouse_pos;
        self.axis_b = glam::Vec2::zero();
    }
}

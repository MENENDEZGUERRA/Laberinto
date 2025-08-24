use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, VirtualKeyCode};

#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub back: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,

    pub mouse_captured: bool,
    pub mouse_dx: f32,
    pub last_mouse_pos: Option<PhysicalPosition<f64>>,
}

pub fn handle_keyboard_input(input: &mut InputState, key: VirtualKeyCode, state: ElementState) {
    let pressed = state == ElementState::Pressed;
    match key {
        VirtualKeyCode::W => input.forward = pressed,
        VirtualKeyCode::S => input.back = pressed,
        VirtualKeyCode::A => input.strafe_left = pressed,
        VirtualKeyCode::D => input.strafe_right = pressed,
        VirtualKeyCode::Left => input.rotate_left = pressed,
        VirtualKeyCode::Right => input.rotate_right = pressed,
        _ => {}
    }
}

pub fn handle_mouse_move(input: &mut InputState, pos: PhysicalPosition<f64>) {
    if !input.mouse_captured {
        input.last_mouse_pos = Some(pos);
        return;
    }
    if let Some(last) = input.last_mouse_pos {
        input.mouse_dx += (pos.x - last.x) as f32;
    }
    input.last_mouse_pos = Some(pos);
}

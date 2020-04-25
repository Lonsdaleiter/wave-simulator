use std::intrinsics::transmute;
use winit::event::VirtualKeyCode;

pub struct Keyboard {
    pub keys: [bool; 300],
}

impl Keyboard {
    pub fn set_key(&mut self, keycode: VirtualKeyCode, down: bool) {
        self.keys[unsafe { transmute::<VirtualKeyCode, u32>(keycode) } as usize] = down;
    }
    pub fn is_key_down(&self, keycode: VirtualKeyCode) -> bool {
        self.keys[unsafe { transmute::<VirtualKeyCode, u32>(keycode) } as usize]
    }
}

use winit::event::VirtualKeyCode;

pub struct Keyboard {
    pub keys: [bool; 300],
}

impl Keyboard {
    pub fn set_key(&mut self, keycode: VirtualKeyCode, down: bool) {
        self.keys[keycode as u32 as usize] = down;
    }
    pub fn is_key_down(&self, keycode: VirtualKeyCode) -> bool {
        self.keys[keycode as u32 as usize]
    }
}

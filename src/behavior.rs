use crate::app::Application;
use winit::event::{ElementState, VirtualKeyCode};

pub trait Behavior<T: Application> {
    fn init(&self, state: &mut T);
    fn update(&self, state: &mut T) -> Option<Box<dyn Behavior<T>>>;
    fn draw(&self, state: &mut T);
    fn on_resize(&self, state: &mut T, size: (u32, u32));
    fn on_death(&self, state: &mut T);
    fn on_keyboard_update(&self, state: &mut T, key: VirtualKeyCode, el_state: ElementState);
}

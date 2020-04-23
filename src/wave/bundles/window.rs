use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::{Window, WindowBuilder};
use crate::wave::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct WindowBundle {
    pub window: Window,
}

impl WindowBundle {
    pub fn new(event_loop: &EventLoop<()>) -> WindowBundle {
        WindowBundle {
            window: WindowBuilder::new()
                .with_titlebar_transparent(true)
                .with_title("Wave Simulator")
                .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
                .with_resizable(true)
                .build(&event_loop)
                .unwrap(),
        }
    }
}

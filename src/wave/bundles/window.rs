use winit::event_loop::EventLoop;
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::{Window, WindowBuilder};
use winit::dpi::PhysicalSize;

pub struct WindowBundle {
    pub window: Window,
}

impl WindowBundle {
    pub fn new(event_loop: &EventLoop<()>) -> WindowBundle {
        WindowBundle {
            window: WindowBuilder::new()
                .with_titlebar_transparent(true)
                .with_title("Wave Simulator")
                .with_inner_size(PhysicalSize::new(1280, 720))
                .with_resizable(true)
                .build(&event_loop)
                .unwrap(),
        }
    }
}

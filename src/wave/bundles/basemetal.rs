use crate::wave::bundles::window::WindowBundle;
use crate::wave::constants::VSYNC;
use cull_canyon::{
    set_layer_for_raw_window_handle, CAMetalLayer, MTLCommandQueue, MTLCompileOptions, MTLDevice,
    MTLLibrary,
};

pub struct BaseMetalBundle {
    pub device: MTLDevice,
    pub queue: MTLCommandQueue,
    pub library: MTLLibrary,
}

impl BaseMetalBundle {
    pub unsafe fn new(window_bundle: &WindowBundle) -> BaseMetalBundle {
        let devices = {
            #[cfg(target_os = "macos")]
            {
                MTLDevice::copy_all_devices()
            }
            #[cfg(target_os = "ios")]
            {
                vec![MTLDevice::create_system_default_device()]
            }
        };
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let queue = device.new_command_queue();

        let surface = CAMetalLayer::new();
        surface.set_pixel_format(80); // rgba8unorm = 70; bgra8unorm = 80
        surface.set_display_sync_enabled(VSYNC);
        surface.set_device(device.clone());
        surface.set_contents_scale(2.0);
        set_layer_for_raw_window_handle(surface, &window_bundle.window);

        let library = device
            .new_library_with_source(
                std::fs::read_to_string("resources/shaders.metal")
                    .unwrap()
                    .as_str(),
                MTLCompileOptions::new(),
            )
            .unwrap();

        BaseMetalBundle {
            device,
            queue,
            library,
        }
    }
}

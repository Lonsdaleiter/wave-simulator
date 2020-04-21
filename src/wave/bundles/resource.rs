use crate::wave::WaveApp;
use cull_canyon::{
    set_layer_for_raw_window_handle, CAMetalLayer, MTLCommandQueue, MTLCompileOptions, MTLDevice,
    MTLFunction,
};

pub struct ResourceBundle {
    pub device: MTLDevice,
    pub command_queue: MTLCommandQueue,
    pub static_vertex: MTLFunction,
    pub static_fragment: MTLFunction,
}

impl ResourceBundle {
    pub unsafe fn new(wave_app: &WaveApp) -> ResourceBundle {
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

        let command_queue = device.new_command_queue();

        let surface = CAMetalLayer::new();
        surface.set_device(device.clone());
        surface.set_display_sync_enabled(true); // vsync is ON
        surface.set_pixel_format(80); // bgra8unorm
        surface.set_presents_with_transaction(false);
        set_layer_for_raw_window_handle(surface.clone(), &wave_app.window_bundle.window);

        let library = device
            .new_library_with_source(
                std::fs::read_to_string("resources/shaders.metal")
                    .unwrap()
                    .as_str(),
                MTLCompileOptions::new(),
            )
            .unwrap();
        let static_vertex = library.new_function_with_name("vertex_static").unwrap();
        let static_fragment = library.new_function_with_name("fragment_static").unwrap();

        ResourceBundle {
            device,
            command_queue,
            static_vertex,
            static_fragment,
        }
    }
}

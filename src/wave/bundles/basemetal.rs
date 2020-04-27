use crate::wave::bundles::window::WindowBundle;
use crate::wave::constants::VSYNC;
use cull_canyon::{set_layer_for_raw_window_handle, CAMetalLayer, MTLCommandQueue, MTLCompileOptions, MTLDepthStencilDescriptor, MTLDepthStencilState, MTLDevice, MTLLibrary, MTLTextureDescriptor, MTLTexture};

pub struct BaseMetalBundle {
    pub device: MTLDevice,
    pub queue: MTLCommandQueue,
    pub surface: CAMetalLayer,
    pub library: MTLLibrary,
    pub basic_depth: MTLDepthStencilState,
    pub depth_texture: MTLTexture,
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
        surface.set_presents_with_transaction(false);
        set_layer_for_raw_window_handle(surface.clone(), &window_bundle.window);

        let library = device
            .new_library_with_source(
                std::fs::read_to_string("resources/shaders.metal")
                    .unwrap()
                    .as_str(),
                MTLCompileOptions::new(),
            )
            .unwrap();

        let depth_stencil = device.new_depth_stencil_state({
            let desc = MTLDepthStencilDescriptor::new();
            desc.set_depth_write_enabled(true);
            desc.set_depth_compare_function(1); // less
            desc
        });
        let depth_texture = device.new_texture_with_descriptor({
            let desc = MTLTextureDescriptor::new();
            desc.set_width(window_bundle.window.inner_size().width as u64 * 2);
            desc.set_height(window_bundle.window.inner_size().height as u64 * 2);
            desc.set_pixel_format(252);
            desc.set_storage_mode(2);
            desc
        });

        BaseMetalBundle {
            device,
            queue,
            surface,
            library,
            basic_depth: depth_stencil,
            depth_texture,
        }
    }
}

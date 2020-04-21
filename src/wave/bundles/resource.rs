use cull_canyon::{MTLDevice, CAMetalLayer, set_layer_for_raw_window_handle};
use crate::wave::WaveApp;

pub struct ResourceBundle {
    //
}

impl ResourceBundle {
    pub unsafe fn new(wave_app: &WaveApp) -> ResourceBundle {
        let devices = {
            #[cfg(target_os = "macos")] {
                MTLDevice::copy_all_devices()
            }
            #[cfg(target_os = "ios")] {
                vec![MTLDevice::create_system_default_device()]
            }
        };
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let surface = CAMetalLayer::new();
        surface.set_device(device.clone());
        surface.set_display_sync_enabled(true); // vsync is ON
        surface.set_pixel_format(80); // bgra8unorm
        surface.set_presents_with_transaction(false);
        set_layer_for_raw_window_handle(surface.clone(), &wave_app.window_bundle.window);

        ResourceBundle {}
    }
}

use cull_canyon::MTLDevice;

pub struct BaseMetalBundle {
    pub device: MTLDevice,
}

impl BaseMetalBundle {
    pub unsafe fn new() -> BaseMetalBundle {
        let devices = {
            #[cfg(target_os = "macos")] {
                MTLDevice::copy_all_devices()
            }
            #[cfg(target_os = "ios")] {
                vec![MTLDevice::create_system_default_device()]
            }
        };
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();
        
        BaseMetalBundle {
            device,
        }
    }
}

use crate::behavior::Behavior;
use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::WaveApp;

pub struct BaseLoaderBehavior;
impl Behavior<WaveApp> for BaseLoaderBehavior {
    fn init(&self, state: &mut WaveApp) {
        let base_metal_bundle =
            unsafe { BaseMetalBundle::new(&state.window_bundle.as_ref().unwrap()) };

        state.base_metal_bundle = Some(base_metal_bundle);
    }

    fn update(&self, _state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        None
    }

    fn draw(&self, _state: &mut WaveApp) {
        //
    }

    fn on_resize(&self, _state: &mut WaveApp, _size: (u32, u32)) {
        //
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

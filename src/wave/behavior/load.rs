use crate::behavior::Behavior;
use crate::wave::WaveApp;
use crate::wave::bundles::resource::ResourceBundle;

pub struct ResourceLoadBehavior;
impl Behavior<WaveApp> for ResourceLoadBehavior {
    fn init(&self, state: &mut WaveApp) {
        let resource_bundle = unsafe { ResourceBundle::new(&state) };
        state.resource_bundle = Some(resource_bundle);
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state.window_bundle.window.request_redraw();

        None
    }

    fn draw(&self, _state: &mut WaveApp) {
        //
    }

    fn on_resize(&self, _state: &mut WaveApp, _size: (u32, u32)) {
        //
    }

    fn on_destroy(&self, _state: &mut WaveApp) {
        //
    }
}

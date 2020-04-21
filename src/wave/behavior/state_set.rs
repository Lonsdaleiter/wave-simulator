use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct StateSetBehavior;
impl Behavior<WaveApp> for StateSetBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
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

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

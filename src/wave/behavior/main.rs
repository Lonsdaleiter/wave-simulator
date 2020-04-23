use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct MainBehavior;
impl Behavior<WaveApp> for MainBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state
            .window_bundle
            .as_ref()
            .unwrap()
            .window
            .request_redraw();

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

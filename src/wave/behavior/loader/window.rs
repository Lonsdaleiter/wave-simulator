use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct WindowBehavior;
impl Behavior<WaveApp> for WindowBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
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

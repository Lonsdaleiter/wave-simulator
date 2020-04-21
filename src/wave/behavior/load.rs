use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct ResourceLoadBehavior;
impl Behavior<WaveApp> for ResourceLoadBehavior {
    fn update(&self, _state: &mut WaveApp) {
        //
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

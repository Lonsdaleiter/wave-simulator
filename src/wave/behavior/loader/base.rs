use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct BaseLoaderBehavior;
impl Behavior<WaveApp> for BaseLoaderBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
    }

    fn update(&self, _state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        None
    }

    fn draw(&self, _state: &mut WaveApp) {
        //
    }

    fn on_resize(&self, _state: &mut WaveApp, size: (u32, u32)) {
        //
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

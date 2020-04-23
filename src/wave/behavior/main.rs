use crate::behavior::Behavior;
use crate::wave::WaveApp;

pub struct MainBehavior;
impl Behavior<WaveApp> for MainBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        let k = &state.tabs.as_ref().unwrap()[state.tab as usize];
        // for getting around issues with multiple borrows
        let nk: Box<dyn Behavior<WaveApp>> = unsafe { std::mem::transmute_copy(k) };
        nk.update(state);

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

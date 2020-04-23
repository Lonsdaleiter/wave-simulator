use crate::behavior::Behavior;
use crate::wave::bundles::ui::UiBundle;
use crate::wave::WaveApp;

pub struct MainBehavior;
impl Behavior<WaveApp> for MainBehavior {
    fn init(&self, state: &mut WaveApp) {
        state.ui_bundle = Some(unsafe { UiBundle::new(state.base_metal_bundle.as_ref().unwrap()) })
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state
            .window_bundle
            .as_ref()
            .unwrap()
            .window
            .request_redraw();

        state.tabs.iter().for_each(|tab|{
            //
        });

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

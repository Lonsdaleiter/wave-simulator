use crate::behavior::Behavior;
use crate::wave::behavior::main::MainBehavior;
use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::bundles::matrix::MatrixBundle;
use crate::wave::WaveApp;

pub struct BaseLoaderBehavior;
impl Behavior<WaveApp> for BaseLoaderBehavior {
    fn init(&self, state: &mut WaveApp) {
        let base_metal_bundle =
            unsafe { BaseMetalBundle::new(&state.window_bundle.as_ref().unwrap()) };
        let aspect_ratio = {
            let size = state.window_bundle.as_ref().unwrap().window.inner_size();
            size.width as f32 / size.height as f32
        };
        let matrix_bundle = unsafe { MatrixBundle::new(&base_metal_bundle, aspect_ratio) };

        state.base_metal_bundle = Some(base_metal_bundle);
        state.matrix_bundle = Some(matrix_bundle);
    }

    fn update(&self, _state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        Some(Box::new(MainBehavior))
    }

    fn draw(&self, _state: &mut WaveApp) {}

    fn on_resize(&self, _state: &mut WaveApp, _size: (u32, u32)) {}

    fn on_death(&self, _state: &mut WaveApp) {}
}

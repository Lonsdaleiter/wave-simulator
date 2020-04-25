use crate::behavior::Behavior;
use crate::wave::bundles::ui::UiBundle;
use crate::wave::bundles::water::WaterBundle;
use crate::wave::WaveApp;
use cull_canyon::{
    MTLCommandEncoder, MTLRenderPassAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptor,
    MTLRenderPassDescriptor,
};

pub struct MainBehavior;
impl Behavior<WaveApp> for MainBehavior {
    fn init(&self, state: &mut WaveApp) {
        state.ui_bundle = Some(unsafe { UiBundle::new(state.base_metal_bundle.as_ref().unwrap()) });
        state.water = Some(unsafe {
            WaterBundle::generate_water(&state.base_metal_bundle.as_ref().unwrap())
        });
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state
            .window_bundle
            .as_ref()
            .unwrap()
            .window
            .request_redraw();

        let window_size = state.window_bundle.as_ref().unwrap().window.inner_size();
        let yaw = -(window_size.width as f64 - state.mouse_pos.0 / 2.0);
        let pitch =
            (window_size.height as f64 / 2.0) - (window_size.height as f64 - state.mouse_pos.1);

        let pitch = if pitch >= 90.0 { 90.0 } else { pitch };
        let pitch = if pitch <= -90.0 { -90.0 } else { pitch };

        state.matrix_bundle.as_mut().unwrap().camera.pitch = pitch.to_radians() as f32;
        state.matrix_bundle.as_mut().unwrap().camera.yaw = yaw.to_radians() as f32;

        unsafe { state.matrix_bundle.as_ref().unwrap().edit_view() };

        None
    }

    fn draw(&self, state: &mut WaveApp) {
        let bundle = state.base_metal_bundle.as_ref().unwrap();
        let water = state.water.as_ref().unwrap();
        let matrices = state.matrix_bundle.as_ref().unwrap();

        unsafe {
            if let Some(drawable) = bundle.surface.next_drawable() {
                let command_buffer = bundle.queue.new_command_buffer();

                let encoder = command_buffer.new_render_command_encoder_with_descriptor({
                    let desc = MTLRenderPassDescriptor::new();
                    desc.get_color_attachments()
                        .set_object_at_indexed_subscript(0, {
                            let desc = MTLRenderPassColorAttachmentDescriptor::new();
                            desc.set_texture(drawable.get_texture());
                            desc.set_clear_color(0.0, 0.0, 0.0, 1.0);
                            desc.set_load_action(2);
                            desc.set_store_action(1);
                            desc
                        });
                    desc
                });
                encoder.set_render_pipeline_state(water.water_pipeline.clone());
                encoder.set_vertex_buffer(water.water_buffer.clone(), 0, 0);
                encoder.set_vertex_buffer(matrices.projection.clone(), 0, 1);
                encoder.set_vertex_buffer(matrices.view.clone(), 0, 2);

                encoder.draw_indexed_primitives(
                    3,
                    water.indices_count as u64,
                    1,
                    water.water_indices.clone(),
                    0,
                    1,
                    0,
                    0,
                );
                encoder.end_encoding();

                command_buffer.present_drawable(drawable);
                command_buffer.commit();
            }
        };
    }

    fn on_resize(&self, state: &mut WaveApp, size: (u32, u32)) {
        unsafe {
            state
                .matrix_bundle
                .as_ref()
                .unwrap()
                .edit_projection(size.0 as f32 / size.1 as f32)
        };
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

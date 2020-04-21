use crate::behavior::Behavior;
use crate::wave::WaveApp;
use cull_canyon::{
    MTLCommandEncoder, MTLRenderPassAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptor,
    MTLRenderPassDescriptor,
};
use std::os::raw::c_void;

pub struct StateSetBehavior;
impl Behavior<WaveApp> for StateSetBehavior {
    fn init(&self, _state: &mut WaveApp) {
        //
    }

    fn update(&self, state: &mut WaveApp) -> Option<Box<dyn Behavior<WaveApp>>> {
        state.window_bundle.window.request_redraw();

        None
    }

    fn draw(&self, state: &mut WaveApp) {
        let bundle = state.resource_bundle.as_ref().unwrap();
        unsafe {
            if let Some(drawable) = bundle.surface.next_drawable() {
                let command_buffer = bundle.command_queue.new_command_buffer();

                let render_encoder = command_buffer.new_render_command_encoder_with_descriptor({
                    let desc = MTLRenderPassDescriptor::new();
                    let attachments = desc.get_color_attachments();
                    attachments.set_object_at_indexed_subscript(0, {
                        let desc = MTLRenderPassColorAttachmentDescriptor::new();
                        desc.set_clear_color(0.0, 0.0, 0.0, 1.0);
                        desc.set_load_action(2);
                        desc.set_store_action(1);
                        desc.set_texture(drawable.get_texture());
                        desc
                    });
                    desc
                });
                render_encoder.set_render_pipeline_state(bundle.ui_pipeline.clone());
                // TODO set the rest of the stuff here
                render_encoder.set_vertex_buffer(bundle.quad.clone(), 0, 0);
                render_encoder.set_fragment_bytes(
                    [1.0f32, 0.0, 1.0, 1.0].as_ptr() as *const c_void,
                    16,
                    0,
                );
                render_encoder.draw_primitives(3, 0, 6, 1, 0);
                render_encoder.end_encoding();

                command_buffer.present_drawable(drawable);
                command_buffer.commit();
            };
        };
    }

    fn on_resize(&self, _state: &mut WaveApp, _size: (u32, u32)) {
        //
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

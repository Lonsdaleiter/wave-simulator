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
        // unsafe {
        //     let c = state
        //         .resource_bundle
        //         .as_ref()
        //         .unwrap()
        //         .transformation_buffer
        //         .get_contents() as *mut [f32; 16];
        //     let nc = [
        //         1.0f32, 0.0, 0.0, 0.0, // row 1
        //         0.0, 1.0, 0.0, 0.0, // row 2
        //         0.0, 0.0, 1.0, 0.0, // row 3
        //         0.0, 0.0, 0.0, 1.0, // row 4
        //     ];
        //     std::mem::replace(&mut *c, nc);
        // };
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
                render_encoder.set_render_pipeline_state(bundle.text_pipeline.clone());
                render_encoder.set_vertex_buffer(
                    state
                        .terminal_bundle
                        .as_ref()
                        .unwrap()
                        .letter_map
                        .get(&'q')
                        .unwrap()
                        .buffer
                        .clone(),
                    0,
                    0,
                ); // temporary

                let size = state.window_bundle.window.inner_size();
                render_encoder.set_vertex_bytes(
                    [size.width, size.height].as_ptr() as *const c_void,
                    8,
                    2,
                );
                render_encoder.set_fragment_texture(
                    state
                        .terminal_bundle
                        .as_ref()
                        .unwrap()
                        .atlas_texture
                        .clone(),
                    0,
                );
                render_encoder.set_fragment_sampler_state(
                    state.terminal_bundle.as_ref().unwrap().sampler.clone(),
                    0,
                );
                render_encoder.set_fragment_bytes(
                    [0.125f32, 0.76, 0.055].as_ptr() as *const c_void,
                    12,
                    0,
                );
                render_encoder.draw_primitives(3, 0, 12, 1, 0);
                render_encoder.end_encoding();

                command_buffer.present_drawable(drawable);
                command_buffer.commit();
            };
        };
    }

    fn on_resize(&self, state: &mut WaveApp, size: (u32, u32)) {
        unsafe {
            state
                .resource_bundle
                .as_ref()
                .unwrap()
                .surface
                .set_drawable_size(size.0 as f64, size.1 as f64)
        };
    }

    fn on_death(&self, _state: &mut WaveApp) {
        //
    }
}

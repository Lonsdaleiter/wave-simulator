use crate::wave::bundles::basemetal::BaseMetalBundle;
use cull_canyon::{
    MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLVertexDescriptor,
};

pub struct UiBundle {
    pub pipeline: MTLRenderPipelineState,
}

impl UiBundle {
    pub unsafe fn new(bundle: &BaseMetalBundle) -> UiBundle {
        let pipeline = bundle
            .device
            .new_render_pipeline_state_with_descriptor({
                let desc = MTLRenderPipelineDescriptor::new();
                desc.set_vertex_descriptor(MTLVertexDescriptor::new());
                desc.set_vertex_function(bundle.library.new_function_with_name("ui_vert").unwrap());
                desc.set_fragment_function(
                    bundle.library.new_function_with_name("ui_frag").unwrap(),
                );
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(
                        {
                            let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                            desc.set_pixel_format(80); // bgra8unorm
                            desc.set_blending_enabled(true);
                            desc.set_source_rgb_blend_factor(4); // src alpha
                            desc.set_destination_rgb_blend_factor(5); // 1 - src alpha
                            desc.set_source_alpha_blend_factor(1); // 1
                            desc.set_destination_alpha_blend_factor(5); // 1 - src alpha
                            desc
                        },
                        0,
                    );
                desc
            })
            .unwrap();
        UiBundle { pipeline }
    }
}

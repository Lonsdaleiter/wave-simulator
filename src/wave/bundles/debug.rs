use crate::wave::bundles::basemetal::BaseMetalBundle;
use cull_canyon::{MTLBuffer, MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLVertexDescriptor};
use std::os::raw::c_void;

pub struct DebugBundle {
    pub vertices: MTLBuffer,
    pub indices: MTLBuffer,
    pub pipeline: MTLRenderPipelineState,
}

impl DebugBundle {
    pub unsafe fn new(bundle: &BaseMetalBundle) -> DebugBundle {
        let vertices = [
            -0.5f32, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5,
            -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5,
            -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5,
            0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, -0.5, -0.5,
            0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5,
        ];
        let indices = [
            0u32, 1, 3, 3, 1, 2, 4, 5, 7, 7, 5, 6, 8, 9, 11, 11, 9, 10, 12, 13, 15, 15, 13, 14, 16,
            17, 19, 19, 17, 18, 20, 21, 23, 23, 21, 22,
        ];
        let vertices = bundle.device.new_buffer_with_bytes(
            vertices.as_ptr() as *const c_void,
            vertices.len() as u64 * 4,
            0,
        );
        let indices = bundle.device.new_buffer_with_bytes(
            indices.as_ptr() as *const c_void,
            indices.len() as u64 * 4,
            0,
        );
        let pipeline = bundle
            .device
            .new_render_pipeline_state_with_descriptor({
                let desc = MTLRenderPipelineDescriptor::new();
                desc.set_vertex_function(
                    bundle
                        .library
                        .new_function_with_name("static_vert")
                        .unwrap(),
                );
                desc.set_fragment_function(
                    bundle
                        .library
                        .new_function_with_name("static_frag")
                        .unwrap(),
                );
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(
                        {
                            let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                            desc.set_pixel_format(80);
                            desc
                        },
                        0,
                    );
                desc.set_vertex_descriptor(MTLVertexDescriptor::new());
                desc
            })
            .unwrap();
        DebugBundle {
            vertices,
            indices,
            pipeline,
        }
    }
}

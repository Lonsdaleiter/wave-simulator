use crate::wave::bundles::basemetal::BaseMetalBundle;
use cull_canyon::{
    MTLBuffer, MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLVertexDescriptor,
};
use std::os::raw::c_void;

pub struct WaterBundle {
    pub water_pipeline: MTLRenderPipelineState,
    pub water_buffer: MTLBuffer,
    pub water_indices: MTLBuffer,
}

impl WaterBundle {
    pub unsafe fn generate_water(bundle: &BaseMetalBundle) -> WaterBundle {
        // row by row generation
        const VERTEX_COUNT: u32 = 100;
        let vertices: [f32; (VERTEX_COUNT * VERTEX_COUNT) as usize] = *((0..VERTEX_COUNT)
            .map(|z: u32| {
                (0..VERTEX_COUNT)
                    .map(|x: u32| [x as f32, 0.0, z as f32])
                    .collect::<Vec<[f32; 3]>>()
            })
            .collect::<Vec<Vec<[f32; 3]>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<[f32; 3]>>()
            .as_ptr()
            as *const [f32; (VERTEX_COUNT * VERTEX_COUNT) as usize]);

        const INDICES_COUNT: usize = (6 * (VERTEX_COUNT - 1) * (VERTEX_COUNT - 1)) as usize;
        let mut indices: [u32; INDICES_COUNT] = [0; INDICES_COUNT];
        let mut pointer = 0;
        (0..VERTEX_COUNT - 1).for_each(|z| {
            (0..VERTEX_COUNT - 1).for_each(|x| {
                let top_left = z * VERTEX_COUNT + x;
                let top_right = top_left + 1;
                let bottom_left = (z + 1) * VERTEX_COUNT + x;
                let bottom_right = bottom_left + 1;
                indices[pointer] = top_left;
                pointer += 1;
                indices[pointer] = bottom_left;
                pointer += 1;
                indices[pointer] = top_right;
                pointer += 1;
                indices[pointer] = top_right;
                pointer += 1;
                indices[pointer] = bottom_left;
                pointer += 1;
                indices[pointer] = bottom_right;
                pointer += 1;
            });
        });

        let water_pipeline = bundle
            .device
            .new_render_pipeline_state_with_descriptor({
                let desc = MTLRenderPipelineDescriptor::new();
                desc.get_color_attachments()
                    .set_object_at_indexed_subscript(
                        {
                            let desc = MTLRenderPipelineColorAttachmentDescriptor::new();
                            desc.set_pixel_format(80); // bgra8unorm
                            desc
                        },
                        0,
                    );
                desc.set_vertex_function(
                    bundle.library.new_function_with_name("water_vert").unwrap(),
                );
                desc.set_fragment_function(
                    bundle.library.new_function_with_name("water_frag").unwrap(),
                );
                desc.set_depth_attachment_pixel_format(252); // depth 32 float
                desc.set_vertex_descriptor(MTLVertexDescriptor::new());
                desc
            })
            .unwrap();

        WaterBundle {
            water_pipeline,
            water_buffer: bundle.device.new_buffer_with_bytes(
                vertices.as_ptr() as *const c_void,
                vertices.len() as u64 * 4,
                0,
            ),
            water_indices: bundle.device.new_buffer_with_bytes(
                indices.as_ptr() as *const c_void,
                indices.len() as u64 * 4,
                0,
            ),
        }
    }
}

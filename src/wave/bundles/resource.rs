use crate::wave::WaveApp;
use cull_canyon::{
    set_layer_for_raw_window_handle, CAMetalLayer, MTLBuffer, MTLCommandQueue, MTLCompileOptions,
    MTLDevice, MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLVertexAttributeDescriptor, MTLVertexBufferLayoutDescriptor,
    MTLVertexDescriptor,
};
use std::os::raw::c_void;

pub struct Letters {
    pub caret: MTLBuffer,
}

pub struct ResourceBundle {
    pub device: MTLDevice,
    pub command_queue: MTLCommandQueue,
    pub surface: CAMetalLayer,
    pub ui_pipeline: MTLRenderPipelineState,
    pub quad: MTLBuffer,
    pub transformation_buffer: MTLBuffer,
    pub letters: Letters,
}

impl ResourceBundle {
    pub unsafe fn new(wave_app: &WaveApp) -> ResourceBundle {
        let devices = {
            #[cfg(target_os = "macos")]
            {
                MTLDevice::copy_all_devices()
            }
            #[cfg(target_os = "ios")]
            {
                vec![MTLDevice::create_system_default_device()]
            }
        };
        let device = devices.into_iter().find_map(|d| Some(d)).unwrap();

        let command_queue = device.new_command_queue();

        let q_data = [
            // triangle 1
            -1.0f32, -1.0, 0.0, 1.0, // v1
            -1.0, 1.0, 0.0, 1.0, // v2
            1.0, 1.0, 0.0, 1.0, // v3
            // triangle 2
            1.0, 1.0, 0.0, 1.0, // v3
            1.0, -1.0, 0.0, 1.0, // v4
            -1.0f32, -1.0, 0.0, 1.0, // v1
        ];
        let quad = device.new_buffer_with_bytes(
            q_data.as_ptr() as *const c_void,
            4 * q_data.len() as u64,
            0,
        );

        // write-combined cpu-cache
        // stores 16 f32s
        let transformation_buffer = device.new_buffer_with_length(64, 1);

        let surface = CAMetalLayer::new();
        surface.set_device(device.clone());
        surface.set_display_sync_enabled(true); // vsync is ON
        surface.set_pixel_format(80); // bgra8unorm
        surface.set_presents_with_transaction(false);
        set_layer_for_raw_window_handle(surface.clone(), &wave_app.window_bundle.window);

        let library = device
            .new_library_with_source(
                std::fs::read_to_string("resources/shaders.metal")
                    .unwrap()
                    .as_str(),
                MTLCompileOptions::new(),
            )
            .unwrap();

        let ui_pipeline = {
            let vertex = library.new_function_with_name("vertex_ui").unwrap();
            let fragment = library.new_function_with_name("fragment_ui").unwrap();
            device
                .new_render_pipeline_state_with_descriptor({
                    let desc = MTLRenderPipelineDescriptor::new();
                    desc.set_vertex_function(vertex);
                    desc.set_fragment_function(fragment);
                    desc.set_vertex_descriptor({
                        let desc = MTLVertexDescriptor::new();
                        let layouts = desc.get_layouts();
                        layouts.set_object_at_indexed_subscript(
                            {
                                let desc = MTLVertexBufferLayoutDescriptor::new();
                                desc.set_stride(16);
                                desc.set_step_function(1); // per-vertex
                                desc
                            },
                            0,
                        );

                        let attribs = desc.get_attributes();
                        attribs.set_object_at_indexed_subscript(
                            {
                                let desc = MTLVertexAttributeDescriptor::new();
                                desc.set_buffer_index(0);
                                desc.set_offset(0);
                                desc.set_format(31); // float4
                                desc
                            },
                            0,
                        );

                        desc
                    });
                    let color_attachments = desc.get_color_attachments();
                    color_attachments.set_object_at_indexed_subscript(
                        {
                            let color_attachment =
                                MTLRenderPipelineColorAttachmentDescriptor::new();
                            color_attachment.set_blending_enabled(true);
                            color_attachment.set_source_rgb_blend_factor(4); // src alpha
                            color_attachment.set_destination_rgb_blend_factor(5); // 1 - src alpha
                            color_attachment.set_source_alpha_blend_factor(1); // 1
                            color_attachment.set_destination_alpha_blend_factor(5); // 1 - src alpha
                            color_attachment.set_pixel_format(80); // bgra8unorm
                            color_attachment
                        },
                        0,
                    );
                    desc
                })
                .unwrap()
        };

        fn generate_line(from: (f32, f32), to: (f32, f32), width: f32) -> [f32; 24] {
            let width = width / 2.0;
            [
                // triangle 1
                from.0, from.1 + width, 0.0, 1.0, // v1
                from.0, from.1 - width, 0.0, 1.0, // v2
                to.0, to.1 - width, 0.0, 1.0, // v3
                // triangle 2
                to.0, to.1 - width, 0.0, 1.0, // v3
                to.0, to.1 + width, 0.0, 1.0, // v4
                from.0, from.1 + width, 0.0, 1.0 // v1
            ]
        }

        // draw these with line strips
        let letters = {
            // let caret_data = [
            //     -1.0f32, 1.0, 0.0, 1.0, // p1
            //     1.0, 0.0, 0.0, 1.0, // p3
            //     -1.0, -1.0, 0.0, 1.0, // p3
            // ];
            let caret_data: Vec<f32> = {
                let k = generate_line((-1.0, 1.0), (1.0, 0.0), 0.05);
                let j = generate_line((-1.0, -1.0), (1.0, 0.0), 0.05);
                [k, j].concat()
            };
            let caret = device.new_buffer_with_bytes(
                caret_data.as_ptr() as *const c_void,
                caret_data.len() as u64 * 4,
                0,
            );
            Letters {
                caret,
            }
        };

        ResourceBundle {
            device,
            command_queue,
            surface,
            ui_pipeline,
            quad,
            transformation_buffer,
            letters,
        }
    }
}

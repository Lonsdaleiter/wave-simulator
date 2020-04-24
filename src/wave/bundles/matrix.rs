use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::constants::new_projection_matrix;
use cull_canyon::MTLBuffer;
use std::os::raw::c_void;
use crate::wave::view::Camera;

pub struct MatrixBundle {
    pub projection: MTLBuffer,
    pub view: MTLBuffer,
}

impl MatrixBundle {
    pub unsafe fn new(bundle: &BaseMetalBundle, aspect_ratio: f32) -> MatrixBundle {
        let projection = new_projection_matrix(aspect_ratio);
        let view = [
            1.0f32, 0.0, 0.0, 0.0, // r1
            0.0, 1.0, 0.0, 0.0, // r2
            0.0, 0.0, 1.0, 0.0, // r3
            0.0, 0.0, 0.0, 1.0, // r4
        ];
        MatrixBundle {
            projection: bundle.device.new_buffer_with_bytes(
                projection.as_ptr() as *const c_void,
                projection.len() as u64 * 4,
                0,
            ),
            view: bundle.device.new_buffer_with_bytes(
                view.as_ptr() as *const c_void,
                view.len() as u64 * 4,
                0,
            ),
        }
    }
    pub unsafe fn edit_projection(&self, aspect_ratio: f32) {
        let projection = new_projection_matrix(aspect_ratio);
        let contents = self.projection.get_contents() as *mut [f32; 16];
        std::mem::replace(&mut *contents, projection);
    }
    pub unsafe fn edit_view(&self, camera: Camera) {
        //
    }
}

use crate::wave::bundles::basemetal::BaseMetalBundle;
use crate::wave::constants::new_projection_matrix;
use crate::wave::camera::Camera;
use cull_canyon::MTLBuffer;
use std::os::raw::c_void;

pub struct MatrixBundle {
    pub projection: MTLBuffer,
    pub view: MTLBuffer,
    pub camera: Camera,
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
            camera: Camera {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0,
            },
        }
    }
    pub unsafe fn edit_projection(&self, aspect_ratio: f32) {
        let projection = new_projection_matrix(aspect_ratio);
        let contents = self.projection.get_contents() as *mut [f32; 16];
        std::mem::replace(&mut *contents, projection);
    }
    pub unsafe fn edit_view(&self) {
        let view = self.camera.get_matrix();
        let contents = self.view.get_contents() as *mut [f32; 16];
        std::mem::replace(&mut *contents, view);
    }
}

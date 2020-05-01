use crate::wave::camera::Camera;
use cgmath::{SquareMatrix, Transform};
use cull_canyon::MTLTexture;

pub fn cast_ray(
    mouse_pos: (f64, f64),
    display_size: (u32, u32),
    projection_matrix: cgmath::Matrix4<f32>,
    camera: &Camera,
    water: &MTLTexture,
) {
    let clip_coords = cgmath::Vector3 {
        x: (mouse_pos.0 * 2.0) as f32 / display_size.0 as f32,
        y: (mouse_pos.1 * 2.0) as f32 / display_size.1 as f32,
        z: -1.0,
    };
    let inverted_proj = projection_matrix.invert().unwrap();
    let eye_coords = inverted_proj.transform_vector(clip_coords);
}

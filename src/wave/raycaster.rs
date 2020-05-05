use crate::wave::camera::Camera;
use cgmath::{SquareMatrix, Transform, Vector3};
use cull_canyon::MTLTexture;

pub fn cast_ray(
    mouse_pos: (f64, f64),
    display_size: (u32, u32),
    projection_matrix: cgmath::Matrix4<f32>,
    camera: &Camera,
    water: MTLTexture,
) -> Vector3<f32> {
    let clip_coords = cgmath::Vector3 {
        x: (mouse_pos.0 * 2.0) as f32 / display_size.0 as f32,
        y: (mouse_pos.1 * 2.0) as f32 / display_size.1 as f32,
        z: -1.0,
    };
    let inverted_proj = projection_matrix.invert().unwrap();
    let eye_coords = inverted_proj.transform_vector(clip_coords);
    let eye_coords = cgmath::Vector3 {
        x: eye_coords.x,
        y: eye_coords.y,
        z: -1.0,
    };
    let inverted_view = camera.get_matrix().invert().unwrap();
    let ray = inverted_view.transform_vector(eye_coords);
    get_point_on_ray(camera, ray, 10.0)
}

pub fn get_point_on_ray(
    cam: &Camera,
    ray: cgmath::Vector3<f32>,
    distance: f32,
) -> cgmath::Vector3<f32> {
    let scaled_ray = cgmath::Vector3 {
        x: ray.x * distance,
        y: ray.y * distance,
        z: ray.z * distance,
    };
    let start = cgmath::Vector3 {
        x: cam.x,
        y: cam.y,
        z: cam.z,
    };
    start + scaled_ray
}

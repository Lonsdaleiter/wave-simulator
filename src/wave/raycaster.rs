use crate::wave::camera::Camera;
use crate::wave::constants::{MAX_RAYCAST_DISTANCE, RAYCAST_CLOSENESS_REQ, RAYCAST_RES};
use cgmath::{SquareMatrix, Transform, Vector3};

pub fn cast_ray(projection_matrix: cgmath::Matrix4<f32>, camera: &Camera) -> Option<Vector3<f32>> {
    // let clip_coords = Vector3 {
    //     x: ((mouse_pos.0 * 2.0) as f32 / display_size.0 as f32) - 1.0,
    //     y: -(((mouse_pos.1 * 2.0) as f32 / display_size.1 as f32) - 1.0),
    //     z: -1.0,
    // };
    let clip_coords = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let inverted_proj = projection_matrix.invert().unwrap();
    let eye_coords = inverted_proj.transform_vector(clip_coords);
    let eye_coords = Vector3 {
        x: eye_coords.x,
        y: eye_coords.y,
        z: -1.0,
    };
    let inverted_view = camera.get_matrix().invert().unwrap();
    let ray = inverted_view.transform_vector(eye_coords);
    // Some(get_point_on_ray(camera, ray, 10.0))
    search(ray, camera)
}

fn search(ray: Vector3<f32>, cam: &Camera) -> Option<Vector3<f32>> {
    let mut the_point: Option<Vector3<f32>> = None;
    (0..RAYCAST_RES).for_each(|index| {
        let point = get_point_on_ray(
            cam,
            ray,
            (index * MAX_RAYCAST_DISTANCE) as f32 / RAYCAST_RES as f32,
        );
        if point.y.abs() <= RAYCAST_CLOSENESS_REQ {
            the_point = Some(point);
            return;
        }
    });

    the_point
}

fn get_point_on_ray(cam: &Camera, ray: Vector3<f32>, distance: f32) -> Vector3<f32> {
    let scaled_ray = Vector3 {
        x: ray.x * distance,
        y: ray.y * distance,
        z: ray.z * distance,
    };
    let start = Vector3 {
        x: cam.x,
        y: cam.y,
        z: cam.z,
    };
    start + scaled_ray
}

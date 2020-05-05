use cgmath::{Matrix4, Vector3, SquareMatrix, Deg};

pub fn generate_transformation(
    location: Vector3<f32>,
    rotation: (f32, f32, f32),
    scale: (f32, f32, f32),
) -> Matrix4<f32> {
    let matrix = Matrix4::identity();
    let matrix = matrix * Matrix4::from_translation(location);
    let matrix = matrix * Matrix4::from_axis_angle(Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }, Deg(rotation.0));
    let matrix = matrix * Matrix4::from_axis_angle(Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }, Deg(rotation.1));
    let matrix = matrix * Matrix4::from_axis_angle(Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }, Deg(rotation.2));
    let matrix = matrix * Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2);
    matrix
}
